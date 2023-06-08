#![allow(unused_unsafe)]
#![allow(clippy::cast_lossless)]
use crate::utils::stack::Stack;

use super::{Address, JmpMode, Lit, Opcode, Reg, StackValue, VmStack, callstack::{CallStack, CallFrame}};
use std::cmp::Ordering;

macro_rules! read_bytes {
	($name:ident, $t:tt, $s:literal) => {
		#[allow(dead_code)]
		fn $name(&mut self) -> $t {
			let bytes = &self.program[self.pc..(self.pc + $s)];
			self.pc += $s;
			$t::from_le_bytes(bytes.try_into().unwrap())
		}
	};
}

type Program = Vec<u8>;
pub type Register = StackValue;

pub struct Vm {
	cmp_reg: Ordering,
	program: Program,
	pc: usize,
	stack: VmStack,
	call_stack: CallStack
}
//pc: *const u8,
//start: *const u8

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.is_empty());
		Self {
			cmp_reg: Ordering::Equal,
			program,
			pc: 0,
			stack: VmStack::new(),
			call_stack: CallStack::new()
		}
	}

	// maybe trait
	pub fn run(&mut self) {
		loop {
			let op = std::ptr::addr_of!(self.program[self.pc]);
			let op = unsafe { op.cast::<Opcode>().read_unaligned() };

			// println!("{:?}", self.program);
			// println!("{op:?}");
			// println!("{}\n", self.pc);

			self.pc += 1;
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
					#[allow(clippy::cast_possible_wrap)]
					let ms = since_the_epoch.as_millis() as Lit;
					let reg = self.read_reg();
					self.set_register(reg, Register::Int(ms));
				}
				Opcode::Call => {
					let address = self.read_address();
					let arg_count = self.read_u8() as usize;
					self.call_stack.push(CallFrame::new(self.pc, arg_count, self.stack.len() - arg_count));
					self.pc = address as usize;
				}
				Opcode::Ret => {
					let reg = self.read_reg();
					let ret_value = self.get_register(reg);
					self.stack.push(ret_value);

					let callframe = self.call_stack.pop();
					self.pc = callframe.ret_pc;
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
				Opcode::GetArg => {
					let reg = self.read_reg();
					let i = self.read_u8() as usize;
					let frame = self.call_stack.last();
					let val = self.stack.get(frame.arg0_i + i);

					// println!("[GetArg] val: ({val:?}), reg: {reg}");

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

	#[inline]
	fn get_register(&self, reg: Reg) -> Register {
		let frame = self.call_stack.last();
		frame.registers[reg as usize]
	}

	#[inline]
	fn set_register(&mut self, reg: Reg, val: Register) {
		let mut frame = self.call_stack.last_mut();
		frame.registers[reg as usize] = val;
	}

	#[inline(always)]
	fn op(&mut self, op: fn(Register, Register) -> Register) {
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();
		let dst = self.read_reg();

		self.set_register(dst, op(self.get_register(reg_1), self.get_register(reg_2))); // TODO: handle overflow
	}

	#[inline]
	fn jump(&mut self, cond: fn(Ordering) -> bool) {
		let mode = self.read_u8();
		let mode = std::ptr::addr_of!(mode);
		let mode = unsafe { mode.cast::<JmpMode>().read_unaligned() };

		let address = self.read_address();
		if cond(self.cmp_reg) {
			match mode {
				JmpMode::Absolute => self.pc = address as usize,
				JmpMode::RelativeBackward => self.pc -= address as usize,
				JmpMode::RelativeForward => self.pc += address as usize
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
		self.pc += 1;
		self.program[self.pc - 1]
	}

	read_bytes!(read_u16, u16, 2);
	read_bytes!(read_i16, i16, 2);

	read_bytes!(read_u32, u32, 4);
	read_bytes!(read_i32, i32, 4);

	read_bytes!(read_u64, u64, 8);
	read_bytes!(read_i64, i64, 8);
}
