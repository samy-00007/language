#![allow(clippy::cast_lossless)]
#![allow(clippy::pedantic)]
mod callstack;
pub mod instructions;
pub mod program;
pub mod stack;

use callstack::{CallFrame, CallStack, CALL_STACK_SIZE};
use instructions::{Address, JmpMode, Lit, Opcode, Reg, StackValue, VmStack};
use program::Program;

use crate::utils::stack::Stack;
use std::cmp::Ordering;

macro_rules! read_bytes {
	($name:ident, $t:tt, $s:literal) => {
		#[allow(dead_code)]
		fn $name(&mut self) -> $t {
			let bytes = unsafe { std::slice::from_raw_parts(self.pc(), $s) };
			self.add_to_pc($s);
			$t::from_le_bytes(bytes.try_into().unwrap())
		}
	};
}

pub type Register = StackValue;

#[derive(Debug)]
pub struct Vm {
	cmp_reg: Ordering,
	program: Program,
	stack: VmStack,
	call_stack: CallStack<CALL_STACK_SIZE>,
	current_frame: *mut CallFrame
}

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.code.is_empty());
		Self {
			cmp_reg: Ordering::Equal,
			program,
			stack: VmStack::default(),
			call_stack: CallStack::default(),
			current_frame: std::ptr::null_mut() as *mut CallFrame
		}
	}

	// maybe trait
	pub fn run(&mut self) {
		self.update_current_frame();
		self.stack.preset_up_to(150); // preallocate 10 registers, as of now, they are not allocated automatically
		unsafe {
			(*self.current_frame).pc = self.program.code.as_ptr();
			(*self.current_frame).base = self.program.code.as_ptr();
		}

		loop {
			let op = self.pc();
			let op = unsafe { op.cast::<Opcode>().read_unaligned() };

			// println!("{:?}", self.program);
			//let depth = "|".repeat(self.call_stack.len() - 1);
			//println!("{} {op:?}", depth);
			//println!("{} {}\n", depth, unsafe { (*self.current_frame).pc() });

			self.increment_pc();
			match op {
				Opcode::Halt => break,
				Opcode::Nop => {}
				Opcode::Load => {
					let reg = self.read_reg();
					let val = Register::Int(self.read_lit());
					self.set_register(reg, val);
				}
				Opcode::Move => {
					let src = self.read_reg();
					let dst = self.read_reg();

					self.set_register(dst, self.get_register(src));
				}
				Opcode::Jmp => self.jump(|_| true),
				Opcode::JmpIfTrue => {
					let mode = self.read_u8();
					let mode = std::ptr::addr_of!(mode);
					let mode = unsafe { mode.cast::<JmpMode>().read_unaligned() };

					let reg = self.read_reg();
					let val = self.get_register(reg);
					let StackValue::Bool(cond) = val else { panic!("only bools can be used to jump conditionally (true)"); };

					let address = self.read_address();
					if cond {
						match mode {
							JmpMode::Absolute => self.set_pc(address as usize),
							JmpMode::RelativeBackward => self.remove_from_pc(address as usize),
							JmpMode::RelativeForward => self.add_to_pc(address as usize)
						}
					}
				}
				Opcode::JmpIfFalse => {
					let mode = self.read_u8();
					let mode = std::ptr::addr_of!(mode);
					let mode = unsafe { mode.cast::<JmpMode>().read_unaligned() };

					let reg = self.read_reg();
					let val = self.get_register(reg);
					let StackValue::Bool(cond) = val else { panic!("only bools can be used to jump conditionally (false)"); };

					let address = self.read_address();
					if !cond {
						match mode {
							JmpMode::Absolute => self.set_pc(address as usize),
							JmpMode::RelativeBackward => self.remove_from_pc(address as usize),
							JmpMode::RelativeForward => self.add_to_pc(address as usize)
						}
					}
				}
				Opcode::Add => self.op(std::ops::Add::add),
				Opcode::Sub => self.op(std::ops::Sub::sub),
				Opcode::Mul => self.op(std::ops::Mul::mul),
				Opcode::Div => self.op(std::ops::Div::div),
				Opcode::Lt => self.cmp(Ordering::Less),
				Opcode::Addl => self.op_lit(std::ops::Add::add),
				Opcode::Subl => self.op_lit(std::ops::Sub::sub),
				Opcode::Mull => self.op_lit(std::ops::Mul::mul),
				Opcode::Divl => self.op_lit(std::ops::Div::div),
				Opcode::Ltl => self.cmp_lit(Ordering::Less),
				Opcode::Cmp => {
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();
					self.cmp_reg = self.get_register(reg_1).cmp(&self.get_register(reg_2));
				}
				Opcode::Clock => {
					let now = std::time::SystemTime::now();
					let since_the_epoch = now
						.duration_since(std::time::UNIX_EPOCH)
						.expect("Time went backwards");
					let ms = since_the_epoch.as_millis() as Lit;
					let reg = self.read_reg();
					self.set_register(reg, Register::Int(ms));
				}
				Opcode::Call => {
					let ra = self.read_reg();
					let arg_count = self.read_u8();
					let ret_count = self.read_u8();

					#[cfg(debug_assertions)]
					assert!(ra.checked_add(arg_count).is_some());
					#[cfg(debug_assertions)]
					assert!(ra.checked_add(ret_count).is_some());

					let function = self.get_register(ra);

					let StackValue::Function(func) = function else {
						panic!("Expected function to call in register");
					};

					let base = unsafe { (*self.current_frame).reg0_p }; // TODO: put that in a function

					let frame = CallFrame::new(func, arg_count, ret_count, self.stack.len(), ra);
					self.call_stack.push(frame);
					self.update_current_frame();

					//let to_add = vec![Register::zero(); arg_count + 5]; // preallocate argcount + 5 registers for the function
					//self.stack.append(&to_add);
					self.ensure_register_exists(arg_count + 5);

					for i in 0..arg_count {
						let val = self.raw_get_register(base, ra + 1 + i);
						self.set_register(i, val);
					}
				}
				Opcode::Ret => {
					let ra = self.read_reg();
					let ret_count = self.read_u8();

					let frame = self.call_stack.pop();
					self.update_current_frame();
					let base = frame.reg0_p;
					let ret_reg = frame.ret_reg;

					for i in 0..ret_count {
						let val = self.raw_get_register(base, ra + i); // TODO: maybe don't move the regs, just give the fn access to them
						self.set_register(ret_reg + i, val);
					}
					self.stack.remove(self.stack.len() - base);
				}
				Opcode::LoadF => {
					let reg = self.read_reg();
					let id = self.read_u16() as usize;
					self.set_register(
						reg,
						StackValue::Function(self.program.functions[id].code.as_ptr())
					);
				}
				Opcode::LoadTrue => {
					let reg = self.read_reg();
					self.set_register(reg, Register::Bool(true));
				}
				Opcode::LoadFalse => {
					let reg = self.read_reg();
					self.set_register(reg, Register::Bool(false));
				}
				Opcode::LoadFloat => {
					let reg = self.read_reg();
					let val = self.read_float();
					self.set_register(reg, Register::Float(val));
				}
				Opcode::Push => {
					let reg = self.read_reg();
					self.stack.push(self.get_register(reg));
				}
				Opcode::Pop => {
					let val = self.stack.pop();
					let reg = self.read_reg();
					// println!("[Pop] val: ({val:?}), reg: {reg}");
					self.set_register(reg, val);
				}
				Opcode::Print => {
					let reg = self.read_reg();
					let val = self.get_register(reg);
					println!("[Print] val: ({val:?})");
				}
			}
		}
	}

	fn update_current_frame(&mut self) {
		self.current_frame = self.call_stack.last_mut() as *mut CallFrame;
	}

	fn ensure_register_exists(&mut self, reg: u8) -> usize {
		let base = (unsafe { *self.current_frame }).reg0_p;
		let address = base + reg as usize;

		self.stack.preallocate_up_to(address);
		address
	}

	fn get_register(&self, reg: Reg) -> Register {
		let base = (unsafe { *self.current_frame }).reg0_p;
		self.raw_get_register(base, reg)
	}

	#[inline]
	fn raw_get_register(&self, base: usize, reg: Reg) -> Register {
		self.stack.get(base + reg as usize)
	}

	fn set_register(&mut self, reg: Reg, val: Register) {
		let reg = self.ensure_register_exists(reg); // TODO: remove that

		self.stack.set(reg, val);
	}

	#[inline(always)]
	fn op(&mut self, op: fn(Register, Register) -> Register) {
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();
		let dst = self.read_reg();

		self.set_register(dst, op(self.get_register(reg_1), self.get_register(reg_2))); // TODO: handle overflow
	}

	#[inline(always)]
	fn cmp(&mut self, ord: Ordering) {
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();
		let dst = self.read_reg();

		let cmp = self.get_register(reg_1).cmp(&self.get_register(reg_2));

		self.set_register(dst, StackValue::Bool(cmp == ord)); // TODO: handle overflow
	}

	#[inline(always)]
	fn cmp_lit(&mut self, ord: Ordering) {
		let reg_1 = self.read_reg();
		let val = self.read_lit();
		let dst = self.read_reg();

		let cmp = self.get_register(reg_1).cmp(&StackValue::Int(val));

		self.set_register(dst, StackValue::Bool(cmp == ord)); // TODO: handle overflow
	}

	#[inline(always)]
	fn op_lit(&mut self, op: fn(Register, Register) -> Register) {
		let reg_1 = self.read_reg();
		let val = self.read_lit();
		let dst = self.read_reg();

		self.set_register(dst, op(self.get_register(reg_1), StackValue::Int(val))); // TODO: handle overflow
	}

	#[inline]
	fn jump(&mut self, cond: fn(Ordering) -> bool) {
		let mode = self.read_u8();
		let mode = std::ptr::addr_of!(mode);
		let mode = unsafe { mode.cast::<JmpMode>().read_unaligned() };
		let address = self.read_address();
		if cond(self.cmp_reg) {
			match mode {
				JmpMode::Absolute => self.set_pc(address as usize),
				JmpMode::RelativeBackward => self.remove_from_pc(address as usize),
				JmpMode::RelativeForward => self.add_to_pc(address as usize)
			}
		}
	}

	#[inline(always)]
	fn read_lit(&mut self) -> Lit {
		self.read_i64()
	}

	#[inline]
	fn read_float(&mut self) -> f64 {
		self.read_f64()
	}

	#[inline(always)]
	#[allow(clippy::assertions_on_constants)]
	fn read_reg(&mut self) -> Reg {
		#[cfg(debug_assertions)]
		assert!(Reg::BITS == u8::BITS);
		self.read_u8()
	}

	#[inline(always)]
	fn read_address(&mut self) -> Address {
		self.read_u16()
	}

	#[inline]
	fn read_u8(&mut self) -> u8 {
		let v = unsafe { *self.pc() };
		self.increment_pc();
		v
	}

	#[inline]
	fn pc(&self) -> *const u8 {
		unsafe { (*self.current_frame).pc }
	}

	#[inline(always)]
	fn increment_pc(&mut self) {
		unsafe {
			(*self.current_frame).increment_pc();
		}
	}

	#[inline(always)]
	fn add_to_pc(&mut self, count: usize) {
		unsafe { (*self.current_frame).add_to_pc(count) }
	}

	#[inline(always)]
	fn remove_from_pc(&mut self, count: usize) {
		unsafe { (*self.current_frame).remove_from_pc(count) }
	}

	#[inline(always)]
	fn set_pc(&mut self, count: usize) {
		unsafe { (*self.current_frame).set_pc(count) }
	}

	read_bytes!(read_u16, u16, 2);
	read_bytes!(read_i16, i16, 2);

	read_bytes!(read_u32, u32, 4);
	read_bytes!(read_i32, i32, 4);

	read_bytes!(read_u64, u64, 8);
	read_bytes!(read_i64, i64, 8);

	read_bytes!(read_f64, f64, 8);
}
