pub mod assembler;
pub mod compiler;
pub mod vm;

use std::{ops::{Add, Sub}, cmp::Ordering};

use assembler::Assembler;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum StackValue {
	Int(i128),
	Float(f64),
	Bool(bool)
}

impl Add for StackValue {
	type Output = Self;
	
	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Self::Bool(_) => panic!("Can't add bools (lhs)"),
			Self::Int(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Int(y) => Self::Int(x + y),
				Self::Float(y) => Self::Float(x as f64 + y),
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => Self::Float(x + y),
				Self::Int(y) => Self::Float(x + y as f64)	
			}
		}
	}
}


impl Sub for StackValue {
	type Output = Self;
	
	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Self::Bool(_) => panic!("Can't add bools (lhs)"),
			Self::Int(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Int(y) => Self::Int(x - y),
				Self::Float(y) => Self::Float(x as f64 - y),
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => Self::Float(x - y),
				Self::Int(y) => Self::Float(x - y as f64)	
			}
		}
	}
}

impl StackValue {
	pub fn cmp(self, rhs: &Self) -> Ordering {
		match self {
			Self::Bool(_) => panic!("Can't add bools (lhs)"),
			Self::Int(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Int(y) => x.cmp(y),
				Self::Float(y) => cmp(x as f64, *y),
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => cmp(x, *y),
				Self::Int(y) => cmp(x, *y as f64)	
			}
		}
	}
}


pub fn cmp(a: f64, b: f64) -> Ordering {
	let diff = a - b;
	if diff.abs() < f64::EPSILON {
		Ordering::Equal
	} else if diff > f64::EPSILON {
		Ordering::Greater
	} else {
		Ordering::Less
	}
}


// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

pub type Reg = u8;
pub type Lit = i16;
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
	Cmp,
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
	Cmp(Reg, Reg),
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
				assembler.add_i16(value);
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
			Self::Cmp(reg_1, reg_2) => {
				assembler.add_u8(Opcode::Cmp as u8);
				assembler.add_u8(reg_1);
				assembler.add_u8(reg_2);
			}
			Self::Clock(reg) => {
				assembler.add_u8(Opcode::Clock as u8);
				assembler.add_u8(reg);
			}
		}
	}
}
