#![allow(clippy::cast_lossless)]
#![allow(clippy::pedantic)]
use crate::utils::stack::Stack;

use super::{
	callstack::{CallFrame, CallStack, CALL_STACK_SIZE},
	Address, JmpMode, Lit, Opcode, Reg, StackValue, VmStack, program::Program,
	//program::Program
};
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
	current_frame: *mut CallFrame,
}

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.code.is_empty());
		let mut s = Self {
			cmp_reg: Ordering::Equal,
			program,
			stack: VmStack::new(),
			call_stack: CallStack::default(),
			current_frame: std::ptr::null_mut() as *mut CallFrame,
		};
		s.current_frame = s.call_stack.last_mut() as *mut _;
		unsafe {
			(*s.current_frame).pc = s.program.code.as_ptr();
			(*s.current_frame).base = s.program.code.as_ptr();
		}
		s.ensure_register_exists(10); // preallocate 10 registers, as of now, they are not allocated automatically
		s
	}

	// maybe trait
	pub fn run(&mut self) {		
		loop {			
			let op = self.pc();
			let op = unsafe { op.cast::<Opcode>().read_unaligned() };

			// println!("{:?}", self.program);
			// println!("{op:?}\n");
			// println!("{}\n", self.pc());

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
				Opcode::Jlt => self.jump(Ordering::is_lt),
				Opcode::Jle => self.jump(Ordering::is_le),
				Opcode::Jgt => self.jump(Ordering::is_gt),
				Opcode::Jge => self.jump(Ordering::is_ge),
				Opcode::Add => self.op(std::ops::Add::add),
				Opcode::Sub => self.op(std::ops::Sub::sub),
				Opcode::Mul => self.op(std::ops::Mul::mul),
				Opcode::Div => self.op(std::ops::Div::div),
				Opcode::Addl => self.op_lit(std::ops::Add::add),
				Opcode::Subl => self.op_lit(std::ops::Sub::sub),
				Opcode::Mull => self.op_lit(std::ops::Mul::mul),
				Opcode::Divl => self.op_lit(std::ops::Div::div),
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
					let id = self.read_u16() as usize;
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();

					assert!(reg_2 >= reg_1);
					
					let base = unsafe { (*self.current_frame).reg0_p }; // TODO: put that in a function

					let arg_count = (reg_2 - reg_1) as usize;

					let func = self.program.functions[id].code.as_ptr();

					let frame = CallFrame::new(func, arg_count, self.stack.len(), reg_1);
					self.call_stack.push(frame);
					self.update_current_frame();

					let to_add = vec![Register::zero(); arg_count + 5]; // preallocate argcount + 5 registers for the function
					self.stack.append(&to_add);
					
					for i in 0..(reg_2 - reg_1) {
						let val = self.raw_get_register(base, reg_1 + i);
						self.set_register(i, val);
					}
				}
				Opcode::Ret => {
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();
					
					assert!(reg_2 >= reg_1);

					let frame = self.call_stack.pop();
					self.update_current_frame();
					let base = frame.reg0_p;
					let ret_reg = frame.ret_reg;
					
					for i in 0..(reg_2 - reg_1) {
						let val = self.raw_get_register(base, reg_1 + i);
						self.set_register(ret_reg + i, val);
					}
					self.stack.remove(self.stack.len() - base);

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

	fn ensure_register_exists(&mut self, reg: usize) {
		if reg + 1 > self.stack.len() {
			let to_add = reg + 1 - self.stack.len();
			let to_add = vec![Register::zero(); to_add];
			self.stack.append(&to_add);
		}
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
		let base = (unsafe { *self.current_frame }).reg0_p;
		let reg = base + reg as usize;
		
		self.ensure_register_exists(reg);
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
		unsafe { 
			(*self.current_frame).pc
		}
	}

	#[inline(always)]
	fn increment_pc(&mut self) {
		unsafe {
			(*self.current_frame).increment_pc();
		}
	}
	
	#[inline(always)]
	fn add_to_pc(&mut self, count: usize) {
		unsafe {
			(*self.current_frame).add_to_pc(count)
		}
	}
	
	#[inline(always)]
	fn remove_from_pc(&mut self, count: usize) {
		unsafe {
			(*self.current_frame).remove_from_pc(count)
		}
	}

	#[inline(always)]
	fn set_pc(&mut self, count: usize) {
		unsafe {
			(*self.current_frame).set_pc(count)
		}
	}

	read_bytes!(read_u16, u16, 2);
	read_bytes!(read_i16, i16, 2);

	read_bytes!(read_u32, u32, 4);
	read_bytes!(read_i32, i32, 4);

	read_bytes!(read_u64, u64, 8);
	read_bytes!(read_i64, i64, 8);
}
