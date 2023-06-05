pub mod assembler;
pub mod compiler;
pub mod vm;

use assembler::Assembler;

// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

type Reg = u8;
type Lit = u16;
type Address = u16;

enum Opcode {
	NOP,
	LOAD,
	MOVE,
	JMP,
	JLE,
	ADD,
	SUB,
	CMP
}

// TODO: use derive macro to generate the compile automatically
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
	NOP,
	LOAD(Reg, Lit),
	MOVE { src: Reg, dst: Reg },
	JMP(Address),
	JLE(Address),
	ADD { reg_1: Reg, reg_2: Reg, dst: Reg },
	SUB { reg_1: Reg, reg_2: Reg, dst: Reg },
	CMP(Reg, Reg)
}

impl Instr {
	#[inline]
	pub fn compile(self, assembler: &mut Assembler) {
		match self {
			Self::NOP => {
				assembler.add_u8(Opcode::NOP as u8);
			}
			Self::LOAD(reg, value) => {
				assembler.add_u8(Opcode::LOAD as u8);
				assembler.add_u8(reg);
				assembler.add_u16(value);
			}
			Self::MOVE { src, dst } => {
				assembler.add_u8(Opcode::MOVE as u8);
				assembler.add_u8(src);
				assembler.add_u8(dst);
			}
			Self::JMP(address) => {
				assembler.add_u8(Opcode::JMP as u8);
				assembler.add_u16(address);
			}
			Self::JLE(address) => {
				assembler.add_u8(Opcode::JLE as u8);
				assembler.add_u16(address);
			}
			Self::ADD { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::ADD as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::SUB { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::SUB as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::CMP(reg_1, reg_2) => {
				assembler.add_u8(Opcode::CMP as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
			}
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cmp {
	Eq,
	Less,
	Greater
}

impl Cmp {
	#[inline(always)]
	pub fn is_eq(self) -> bool {
		self == Self::Eq
	}

	#[inline(always)]
	pub fn is_neq(self) -> bool {
		!self.is_eq()
	}

	#[inline(always)]
	pub fn is_gt(self) -> bool {
		self == Self::Greater
	}

	#[inline(always)]
	pub fn is_lt(self) -> bool {
		self == Self::Less
	}

	#[inline(always)]
	pub fn is_gte(self) -> bool {
		!self.is_lt()
	}

	#[inline(always)]
	pub fn is_lte(self) -> bool {
		!self.is_gt()
	}
}
