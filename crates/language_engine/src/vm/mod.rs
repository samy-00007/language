#![allow(clippy::cast_lossless)]
#![allow(clippy::pedantic)]
mod callstack;
// pub mod instructions;
pub mod opcodes;
pub mod program;
pub mod stack;

use callstack::{CallFrame, CallStack, CALL_STACK_SIZE};
use opcodes::{Address, Lit, Opcode, Reg};
use program::Program;
use stack::{StackValue, VmStack};

use crate::utils::stack::Stack;
use std::{cell::RefCell, cmp::Ordering};

macro_rules! impl_reads {
	($name:ident, $t:tt) => {
		#[inline]
		#[allow(dead_code)]
		fn $name(&mut self) -> $t {
			self.current_frame.borrow_mut().$name()
		}
	};
}

pub type Register = StackValue;

#[derive(Debug)]
pub struct Vm {
	program: Program,
	stack: VmStack,
	call_stack: CallStack<CALL_STACK_SIZE>,
	current_frame: RefCell<CallFrame>
}

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.code.is_empty());
		let mut call_stack = CallStack::new();
		let root = CallFrame::new(program.clone(), 0, 0, 0, 0, 0);
		let root = RefCell::new(root);
		call_stack.push(root.clone());
		Self {
			program,
			stack: VmStack::default(),
			call_stack,
			current_frame: root
		}
	}

	// maybe trait
	pub fn run(&mut self) {
		self.update_current_frame();
		self.stack.preset_up_to(150); // preallocate 10 registers, as of now, they are not allocated automatically
		loop {
			#[cfg(debug_assertions)]
			self.current_frame.borrow().ensure_no_overlow();

			let op = self.read_u8().into();

			match op {
				Opcode::Halt => break,
				Opcode::Nop => {}
				Opcode::Load => {
					let reg = self.read_reg();
					let val = Register::Int(self.read_lit());
					self.set_register(reg, val);
				}
				Opcode::Move => {
					let dst = self.read_reg();
					let src = self.read_reg();

					self.set_register(dst, self.get_register(src));
				}
				Opcode::Jmp => {
					let address = self.read_address();
					self.set_pc(address as usize);
				}
				Opcode::JmpIfTrue => {
					let reg = self.read_reg();
					let val = self.get_register(reg);
					let address = self.read_address();

					if val == StackValue::Bool(true) {
						self.set_pc(address as usize);
					}
				}
				Opcode::JmpIfFalse => {
					let reg = self.read_reg();
					let val = self.get_register(reg);
					let address = self.read_address();

					if val == StackValue::Bool(false) {
						self.set_pc(address as usize);
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

					let func = self.program.functions[func as usize].clone();

					let base = self.current_frame.borrow().reg0_p; // TODO: put that in a function

					let frame = CallFrame::new(func, 0, arg_count, ret_count, self.stack.len(), ra);
					self.call_stack.push(RefCell::new(frame));
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
					let frame = frame.borrow();
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
					let id = self.read_u16();
					self.set_register(reg, StackValue::Function(id));
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
				Opcode::Print => {
					let reg = self.read_reg();
					let val = self.get_register(reg);
					println!("[Print] val: ({val:?})");
				}
			}
		}
	}

	fn update_current_frame(&mut self) {
		self.current_frame = self.call_stack.last();
	}

	fn ensure_register_exists(&mut self, reg: u8) -> usize {
		let base = self.current_frame.borrow().reg0_p;
		let address = base + reg as usize;

		self.stack.preallocate_up_to(address);
		address
	}

	fn get_register(&self, reg: Reg) -> Register {
		let base = self.current_frame.borrow().reg0_p;
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
		let dst = self.read_reg();
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();

		self.set_register(dst, op(self.get_register(reg_1), self.get_register(reg_2))); // TODO: handle overflow
	}

	#[inline(always)]
	fn cmp(&mut self, ord: Ordering) {
		let dst = self.read_reg();
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();

		let cmp = self.get_register(reg_1).cmp(&self.get_register(reg_2));

		self.set_register(dst, StackValue::Bool(cmp == ord));
	}

	#[inline(always)]
	fn cmp_lit(&mut self, ord: Ordering) {
		let dst = self.read_reg();
		let reg_1 = self.read_reg();
		let val = self.read_lit();

		let cmp = self.get_register(reg_1).cmp(&StackValue::Int(val));

		self.set_register(dst, StackValue::Bool(cmp == ord));
	}

	#[inline(always)]
	fn op_lit(&mut self, op: fn(Register, Register) -> Register) {
		let dst = self.read_reg();
		let reg_1 = self.read_reg();
		let val = self.read_lit();

		self.set_register(dst, op(self.get_register(reg_1), StackValue::Int(val))); // TODO: handle overflow
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

	#[inline(always)]
	fn set_pc(&mut self, count: usize) {
		self.current_frame.borrow_mut().set_pc(count)
	}

	impl_reads!(read_u8, u8);

	impl_reads!(read_u16, u16);
	impl_reads!(read_i16, i16);

	impl_reads!(read_u32, u32);
	impl_reads!(read_i32, i32);

	impl_reads!(read_u64, u64);
	impl_reads!(read_i64, i64);

	impl_reads!(read_f64, f64);
}
