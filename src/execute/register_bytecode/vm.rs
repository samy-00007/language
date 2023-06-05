#![allow(unused_unsafe)]
#![allow(clippy::cast_lossless)]
use super::{Address, Lit, Opcode, Reg};
use std::cmp::Ordering;

type Program = Vec<u8>;
type Register = u64;

pub struct Vm {
	pub registers: [Register; 256],
	cmp_reg: Ordering,
	program: Program,
	pc: usize //pc: *const u8,
	          //start: *const u8
}

impl Vm {
	pub fn new(program: Program) -> Self {
		assert!(!program.is_empty());
		Self {
			registers: [0; 256],
			cmp_reg: Ordering::Equal,
			program,
			pc: 0 // pc: program.as_ptr(),
			      // start: program.as_ptr()
		}
	}

	// maybe trait
	pub fn run(&mut self) {
		loop {
			//assert!(self.pc <= self.program.len());

			// println!("{:?}", self.program);
			// println!("{:?}", self.pc);
			// let op = unsafe {self.pc.cast::<Opcode>().read_unaligned()};
			// let op = unsafe {
			// 	let op = *self.program.get_unchecked(self.pc) as *const u8;
			// 	op.cast::<Opcode>().read_unaligned()
			// };

			let op = std::ptr::addr_of!(self.program[self.pc]);

			let op = unsafe { op.cast::<Opcode>().read_unaligned() };

			// println!("{op:?}");

			self.pc += 1;
			match op {
				Opcode::Halt => break,
				Opcode::Nop => {}
				Opcode::Load => {
					let reg = self.read_reg();
					let val = self.read_lit() as Register;
					self.registers[reg] = val;
				}
				Opcode::Move => {
					let src = self.read_reg();
					let dst = self.read_reg();

					self.registers[dst] = self.registers[src];
				}
				Opcode::Jmp => {
					self.pc = self.read_address() as usize;
				}
				Opcode::Jlt => {
					// WARNING: assumes self.cmpReg is set beforehand
					let address = self.read_address() as usize;
					if self.cmp_reg.is_lt() {
						self.pc = address;
					}
				}
				Opcode::Jle => {
					// WARNING: assumes self.cmpReg is set beforehand
					let address = self.read_address() as usize;
					if self.cmp_reg.is_le() {
						self.pc = address;
					}
				}
				Opcode::Jgt => {
					// WARNING: assumes self.cmpReg is set beforehand
					let address = self.read_address() as usize;
					if self.cmp_reg.is_gt() {
						self.pc = address;
					}
				}
				Opcode::Jge => {
					// WARNING: assumes self.cmpReg is set beforehand
					let address = self.read_address() as usize;
					if self.cmp_reg.is_ge() {
						self.pc = address;
					}
				}
				Opcode::Add => {
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();
					let dst = self.read_reg();

					self.registers[dst] = self.registers[reg_1] + self.registers[reg_2]; // TODO: handle overflow
				}
				Opcode::Sub => {
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();
					let dst = self.read_reg();

					self.registers[dst] = self.registers[reg_1] - self.registers[reg_2]; // TODO: handle overflow
				}
				Opcode::Cmp => {
					let reg_1 = self.read_reg();
					let reg_2 = self.read_reg();
					self.cmp_reg = self.registers[reg_1].cmp(&self.registers[reg_2]);
				}
			}
		}
	}

	#[inline(always)]
	fn read_lit(&mut self) -> Lit {
		self.read_u16()
	}

	#[inline(always)]
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

	fn read_u16(&mut self) -> u16 {
		let bytes = &self.program[self.pc..(self.pc + 2)];
		self.pc += 2;
		u16::from_le_bytes(bytes.try_into().unwrap())
	}

	#[allow(dead_code)]
	fn read_u32(&mut self) -> u32 {
		let bytes = &self.program[self.pc..(self.pc + 4)];
		self.pc += 4;
		u32::from_le_bytes(bytes.try_into().unwrap())
	}

	#[allow(dead_code)]
	fn read_u64(&mut self) -> u64 {
		let bytes = &self.program[self.pc..(self.pc + 8)];
		self.pc += 8;
		u64::from_le_bytes(bytes.try_into().unwrap())
	}
}
