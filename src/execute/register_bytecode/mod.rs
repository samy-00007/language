pub mod assembler;
pub mod compiler;
mod stack;
pub mod vm;

pub use stack::*;

use assembler::Assembler;

// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

pub type Reg = u8;
pub type Lit = i64;
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
	Mul,
	Div,
	Cmp,
	Call,
	Ret,
	Push,
	Pop,
	Clock
}

// TODO: use derive macro to generate the compile automatically
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instr {
	Halt,
	Nop,
	Load(Reg, Lit),
	Move { src: Reg, dst: Reg },
	Jmp(JmpMode, Address),
	Jlt(JmpMode, Address),
	Jle(JmpMode, Address),
	Jgt(JmpMode, Address),
	Jge(JmpMode, Address),
	Add { reg_1: Reg, reg_2: Reg, dst: Reg },
	Sub { reg_1: Reg, reg_2: Reg, dst: Reg }, // TODO: handle literal, instead of only registers
	Mul { reg_1: Reg, reg_2: Reg, dst: Reg },
	Div { reg_1: Reg, reg_2: Reg, dst: Reg },
	Cmp(Reg, Reg),
	Call(Address),
	Ret,
	Push(Reg),
	Pop(Reg),
	Clock(Reg)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JmpMode {
	Absolute,
	RelativeForward,
	RelativeBackward
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
				assembler.add_i64(value);
			}
			Self::Move { src, dst } => {
				assembler.add_u8(Opcode::Move as u8);
				assembler.add_u8(src);
				assembler.add_u8(dst);
			}
			Self::Jmp(mode, address) => {
				assembler.add_u8(Opcode::Jmp as u8);
				assembler.add_u8(mode as u8);
				assembler.add_u16(address);
			}
			Self::Jle(mode, address) => {
				assembler.add_u8(Opcode::Jle as u8);
				assembler.add_u8(mode as u8);
				assembler.add_u16(address);
			}
			Self::Jlt(mode, address) => {
				assembler.add_u8(Opcode::Jlt as u8);
				assembler.add_u8(mode as u8);
				assembler.add_u16(address);
			}
			Self::Jgt(mode, address) => {
				assembler.add_u8(Opcode::Jgt as u8);
				assembler.add_u8(mode as u8);
				assembler.add_u16(address);
			}
			Self::Jge(mode, address) => {
				assembler.add_u8(Opcode::Jge as u8);
				assembler.add_u8(mode as u8);
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
			Self::Mul { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::Mul as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::Div { reg_1, reg_2, dst } => {
				assembler.add_u8(Opcode::Div as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
				assembler.add_u8(dst);
			}
			Self::Cmp(reg_1, reg_2) => {
				assembler.add_u8(Opcode::Cmp as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
			}
			Self::Call(address) => {
				assembler.add_u8(Opcode::Call as u8);
				assembler.add_u16(address);
			}
			Self::Clock(reg) => {
				assembler.add_u8(Opcode::Clock as u8);
				assembler.add_u8(reg);
			}
			Self::Ret => {
				assembler.add_u8(Opcode::Ret as u8);
			}
			Self::Push(reg) => {
				assembler.add_u8(Opcode::Push as u8);
				assembler.add_u8(reg);
			}
			Self::Pop(reg) => {
				assembler.add_u8(Opcode::Pop as u8);
				assembler.add_u8(reg);
			}
		}
	}

	#[allow(clippy::cast_possible_truncation)]
	pub const fn size(self) -> usize {
		let mut n = 1; // opcode
		n += match self {
			Self::Load(_, _) => Reg::BITS + Lit::BITS,
			Self::Move { src: _, dst: _ } | Self::Cmp(_, _) => 2 * Reg::BITS,
			Self::Jge(_, _) | Self::Jgt(_, _) | Self::Jle(_, _) | Self::Jmp(_, _) => std::mem::size_of::<JmpMode>() as u32 * 8 + Address::BITS,
			Self::Add { reg_1: _, reg_2: _, dst: _ } | Self::Sub { reg_1: _, reg_2: _, dst: _ } | Self::Mul { reg_1: _, reg_2: _, dst: _ } | Self::Div { reg_1: _, reg_2: _, dst: _ } => 3 * Reg::BITS,
			Self::Call(_) => Address::BITS,
			Self::Clock(_) | Self::Pop(_) | Self::Push(_) => Reg::BITS,
			_ => 0
		} / 8;
		n as usize
	}
}
