pub mod assembler;
mod callstack;
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
	GetArg,
	Clock,
	Print
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
	// u8: arg_count
	Call(Address, u8),
	Ret(Reg),
	Push(Reg),
	Pop(Reg),
	GetArg(Reg, u8),
	Clock(Reg),
	Print(Reg)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JmpMode {
	Absolute,
	RelativeForward,
	RelativeBackward
}


macro_rules! match_ops {
	[
		$($name_1:ident);* ;;
		$($name_2:ident; $(($a:ident, $fn:ident, $t:ty)),*);* ;;
		$($name_3:ident; $(($a_:ident, $fn_:ident, $t_:ty)),*);*
		] => {
		pub fn compile(self, assembler: &mut Assembler) {
			match self {
				$(
					Self::$name_1 => {
						assembler.add_u8(Opcode::$name_1 as u8);
					}
				),*

				$(
					Self::$name_2 ( $($a),* ) => {
						assembler.add_u8(Opcode::$name_2 as u8);
						$(
							assembler.$fn($a as $t);
						)*
					}
				),*
				$(
					Self::$name_3 { $($a_),* } => {
						assembler.add_u8(Opcode::$name_3 as u8);
						$(
							assembler.$fn_($a_ as $t_);
						)*
					}
				),*
			}
		}
	};
}

impl Instr {
	match_ops![
		Halt; 
		Nop 
		;;
		Load; (reg, add_u8, u8), (value, add_i64, i64);
		Jmp; (mode, add_u8, u8), (address, add_u16, u16);
		Jle; (mode, add_u8, u8), (address, add_u16, u16);
		Jlt; (mode, add_u8, u8), (address, add_u16, u16);
		Jgt; (mode, add_u8, u8), (address, add_u16, u16);
		Jge; (mode, add_u8, u8), (address, add_u16, u16);
		Cmp; (reg_1, add_u8, u8), (reg_2, add_u8, u8);
		Call; (address, add_u16, u16), (arg_count, add_u8, u8);
		Clock; (reg, add_u8, u8);
		Ret; (reg, add_u8, u8);
		Push; (reg, add_u8, u8);
		Pop; (reg, add_u8, u8);
		Print; (reg, add_u8, u8);
		GetArg; (reg, add_u8, u8), (i, add_u8, u8)
		;;
		Move; (src, add_u8, u8), (dst, add_u8, u8);
		Add; (reg_1, add_u8, u8), (reg_2, add_u8, u8), (dst, add_u8, u8);
		Sub; (reg_1, add_u8, u8), (reg_2, add_u8, u8), (dst, add_u8, u8);
		Mul; (reg_1, add_u8, u8), (reg_2, add_u8, u8), (dst, add_u8, u8);
		Div; (reg_1, add_u8, u8), (reg_2, add_u8, u8), (dst, add_u8, u8)
	];

	#[allow(clippy::cast_possible_truncation)]
	pub const fn size(self) -> usize {
		let mut n = 1; // opcode
		let to_add = match self {
			Self::Load(_, _) => Reg::BITS + Lit::BITS,
			Self::Move { src: _, dst: _ } | Self::Cmp(_, _) => 2 * Reg::BITS,
			Self::Jge(_, _) | Self::Jgt(_, _) | Self::Jle(_, _) | Self::Jmp(_, _) => {
				std::mem::size_of::<JmpMode>() as u32 * 8 + Address::BITS
			}
			Self::Add {
				reg_1: _,
				reg_2: _,
				dst: _
			}
			| Self::Sub {
				reg_1: _,
				reg_2: _,
				dst: _
			}
			| Self::Mul {
				reg_1: _,
				reg_2: _,
				dst: _
			}
			| Self::Div {
				reg_1: _,
				reg_2: _,
				dst: _
			} => 3 * Reg::BITS,
			Self::Call(_, _) => Address::BITS + u8::BITS,
			Self::Clock(_) | Self::Pop(_) | Self::Push(_) | Self::Ret(_) => Reg::BITS,
			Self::GetArg(_, _) => Reg::BITS + u8::BITS,
			_ => 0
		};
		assert!(to_add % 8 == 0);
		n += to_add / 8;
		n as usize
	}
}
