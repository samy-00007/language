#![allow(unused_unsafe)]
#![allow(clippy::cast_lossless)]
use crate::utils::stack::Stack;

use super::{Address, JmpMode, Lit, Opcode, Reg, StackValue, VmStack};
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
type Register = StackValue;

pub struct Vm {
	pub registers: [Register; 256],
	cmp_reg: Ordering,
	program: Program,
	pc: usize,
	stack: VmStack //pc: *const u8,
	                     //start: *const u8
}

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.is_empty());
		Self {
			registers: [StackValue::Int(0); 256],
			cmp_reg: Ordering::Equal,
			program,
			pc: 0,
			stack: VmStack::new() // pc: program.as_ptr(),
			                  // start: program.as_ptr()
		}
	}

	// maybe trait
	pub fn run(&mut self) {
		loop {
			let op = std::ptr::addr_of!(self.program[self.pc]);
			let op = unsafe { op.cast::<Opcode>().read_unaligned() };

			self.pc += 1;
			match op {
				Opcode::Halt => break,
				Opcode::Nop => {}
				Opcode::Load => {
					let reg = self.read_reg();
					let val = Register::Int(self.read_lit());
					self.registers[reg] = val;
				}
				Opcode::Move => {
					let src = self.read_reg();
					let dst = self.read_reg();

					self.registers[dst] = self.registers[src];
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
					self.cmp_reg = self.registers[reg_1].cmp(&self.registers[reg_2]);
				}
				Opcode::Clock => {
					let now = std::time::SystemTime::now();
					let since_the_epoch = now
						.duration_since(std::time::UNIX_EPOCH)
						.expect("Time went backwards");
					#[allow(clippy::cast_possible_wrap)]
					let ms = since_the_epoch.as_millis() as Lit;
					let reg = self.read_reg();
					self.registers[reg] = Register::Int(ms);
				}
				Opcode::Call => {
					self.stack.push(StackValue::Address(self.pc as Address));
					let address = self.read_address();
					self.pc = address as usize;
				}
				Opcode::Ret => {
					let StackValue::Address(address) = (unsafe { self.stack.pop() }) else {
						unreachable!()
					};
					self.pc = address as usize;
				}
				Opcode::Push => {
					let reg = self.read_reg();
					self.stack.push(self.registers[reg]);
				}
				Opcode::Pop => {
					let val = self.stack.pop();
					let reg = self.read_reg();
					self.registers[reg] = val;
				}
			}
		}
	}

	#[inline(always)]
	fn op(&mut self, op: fn(Register, Register) -> Register) {
		let reg_1 = self.read_reg();
		let reg_2 = self.read_reg();
		let dst = self.read_reg();

		self.registers[dst] = op(self.registers[reg_1], self.registers[reg_2]); // TODO: handle overflow
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
	fn read_reg(&mut self) -> usize {
		#[cfg(debug_assertions)]
		assert!(Reg::BITS == u8::BITS);
		self.read_u8() as usize
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
