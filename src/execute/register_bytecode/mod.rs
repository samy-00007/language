pub mod assembler;
pub mod compiler;
pub mod vm;

use assembler::Assembler;

// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

pub type Reg = u8;
pub type Lit = u16;
pub type Address = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
	Halt,
	Nop,
	Load,
	Move,
	Jmp,
	Jlt,
	Jle,
	Jgt,
	Jge,
	Add,
	Sub,
	Cmp
}

// TODO: use derive macro to generate the compile automatically
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
	Halt,
	Nop,
	Load(Reg, Lit),
	Move { src: Reg, dst: Reg },
	Jmp(Address),
	Jlt(Address),
	Jle(Address),
	Jgt(Address),
	Jge(Address),
	Add { reg_1: Reg, reg_2: Reg, dst: Reg },
	Sub { reg_1: Reg, reg_2: Reg, dst: Reg }, // TODO: handle literal, instead of only registers
	Cmp(Reg, Reg)
}

impl Instr {
	#[inline]
	pub fn compile(self, assembler: &mut Assembler) {
		match self {
			Self::Halt => {
				assembler.add_u8(Opcode::Halt as u8);
			}
			Self::Nop => {
				assembler.add_u8(Opcode::Nop as u8);
			}
			Self::Load(reg, value) => {
				assembler.add_u8(Opcode::Load as u8);
				assembler.add_u8(reg);
				assembler.add_u16(value);
			}
			Self::Move { src, dst } => {
				assembler.add_u8(Opcode::Move as u8);
				assembler.add_u8(src);
				assembler.add_u8(dst);
			}
			Self::Jmp(address) => {
				assembler.add_u8(Opcode::Jmp as u8);
				assembler.add_u16(address);
			}
			Self::Jle(address) => {
				assembler.add_u8(Opcode::Jle as u8);
				assembler.add_u16(address);
			}
			Self::Jlt(address) => {
				assembler.add_u8(Opcode::Jlt as u8);
				assembler.add_u16(address);
			}
			Self::Jgt(address) => {
				assembler.add_u8(Opcode::Jgt as u8);
				assembler.add_u16(address);
			}
			Self::Jge(address) => {
				assembler.add_u8(Opcode::Jge as u8);
				assembler.add_u16(address);
			}
			Self::Add { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::Add as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::Sub { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::Sub as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::Cmp(reg_1, reg_2) => {
				assembler.add_u8(Opcode::Cmp as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
			}
		}
	}
}
