use std::fmt::Display;

pub use super::stack::*;

use super::assembler::Assembler;

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
	JmpIfTrue,
	JmpIfFalse,
	Add,
	Sub,
	Mul,
	Div,
	Addl,
	Subl,
	Mull,
	Divl,
	Cmp,
	Call,
	Ret,
	LoadF,
	LoadTrue,
	LoadFalse,
	LoadFloat,
	Push,
	Pop,
	// GetArg,
	Clock,
	Print
}

// TODO: use derive macro to generate the compile automatically
// TODO: standardize the op orders to other assembly languages
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Instr {
	Halt,
	Nop,
	Load(Reg, Lit),
	Move { src: Reg, dst: Reg },
	Jmp(JmpMode, Address),
	JmpIfTrue(JmpMode, Reg, Address),
	JmpIfFalse(JmpMode, Reg, Address),
	Add { reg_1: Reg, reg_2: Reg, dst: Reg },
	Sub { reg_1: Reg, reg_2: Reg, dst: Reg },
	Mul { reg_1: Reg, reg_2: Reg, dst: Reg },
	Div { reg_1: Reg, reg_2: Reg, dst: Reg },
	Addl { reg_1: Reg, val: Lit, dst: Reg },
	Subl { reg_1: Reg, val: Lit, dst: Reg },
	Mull { reg_1: Reg, val: Lit, dst: Reg },
	Divl { reg_1: Reg, val: Lit, dst: Reg },
	Cmp(Reg, Reg),
	// inspired by lua: R[A], R[A+1], ..., R[A+C-1] = R[A](R[A+1], R[A+2], ..., R[A+B])
	// B: num of args, C: num of ret values
	Call(Reg, u8, u8),
	// return R[A], ..., R[A+B-1]
	// B: num of ret values
	Ret(Reg, u8),
	LoadF(Reg, u16), // R[A] = function[id]
	LoadTrue(Reg),
	LoadFalse(Reg),
	LoadFloat(Reg, f64),
	Push(Reg),
	Pop(Reg),
	Clock(Reg),
	Print(Reg)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JmpMode {
	Absolute,
	RelativeForward,
	RelativeBackward
}

impl Display for JmpMode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let str = match self {
			Self::Absolute => "absolute",
			Self::RelativeBackward => "backward",
			Self::RelativeForward => "forward"
		};
		write!(f, "{}", str)
	}
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

		pub fn to_string(self) -> String {
			match self {
				$(
					Self::$name_1 => {
						stringify!($name_1).to_uppercase()
					}
				),*

				$(
					Self::$name_2 ( $($a),* ) => {
						let mut op = stringify!($name_2).to_uppercase();
						$(
							op += format!(" {}", $a).as_str();
						)*
						op
					}
				),*
				$(
					Self::$name_3 { $($a_),* } => {
						let mut op = stringify!($name_3).to_uppercase();
						$(
							op += format!(" {}", $a_).as_str();
						)*
						op
					}
				),*
			}
		}
	};
}

// GetArg; (reg, add_u8, Reg), (i, add_u8, u8)
impl Instr {
	match_ops![
		Halt;
		Nop
		;;
		Load; (reg, add_u8, u8), (value, add_i64, Lit);
		Jmp; (mode, add_u8, u8), (address, add_u16, Address);
		JmpIfTrue; (mode, add_u8, u8), (reg, add_u8, u8), (address, add_u16, Address);
		JmpIfFalse; (mode, add_u8, u8), (reg, add_u8, u8), (address, add_u16, Address);
		Cmp; (reg_1, add_u8, Reg), (reg_2, add_u8, Reg);
		Clock; (reg, add_u8, Reg);
		Push; (reg, add_u8, Reg);
		Pop; (reg, add_u8, Reg);
		Print; (reg, add_u8, Reg);
		Call; (id, add_u8, Reg), (argc, add_u8, u8), (retc, add_u8, u8);
		Ret; (reg_1, add_u8, Reg), (n, add_u8, u8);
		LoadF; (reg, add_u8, Reg), (id, add_u16, u16);
		LoadTrue; (reg, add_u8, Reg);
		LoadFalse; (reg, add_u8, Reg);
		LoadFloat; (reg, add_u8, Reg), (val, add_f64, f64)
		;;
		Move; (src, add_u8, Reg), (dst, add_u8, Reg);
		Add; (reg_1, add_u8, Reg), (reg_2, add_u8, Reg), (dst, add_u8, Reg);
		Sub; (reg_1, add_u8, Reg), (reg_2, add_u8, Reg), (dst, add_u8, Reg);
		Mul; (reg_1, add_u8, Reg), (reg_2, add_u8, Reg), (dst, add_u8, Reg);
		Div; (reg_1, add_u8, Reg), (reg_2, add_u8, Reg), (dst, add_u8, Reg);
		Addl; (reg_1, add_u8, Reg), (val, add_i64, Lit), (dst, add_u8, Reg);
		Subl; (reg_1, add_u8, Reg), (val, add_i64, Lit), (dst, add_u8, Reg);
		Mull; (reg_1, add_u8, Reg), (val, add_i64, Lit), (dst, add_u8, Reg);
		Divl; (reg_1, add_u8, Reg), (val, add_i64, Lit), (dst, add_u8, Reg)
	];

	#[allow(clippy::cast_possible_truncation)]
	pub const fn size(self) -> usize {
		let mut n = 1; // opcode
		let to_add = match self {
			Self::Load(_, _) => Reg::BITS + Lit::BITS,
			Self::Move { src: _, dst: _ } | Self::Cmp(_, _) | Self::Ret(_, _) => 2 * Reg::BITS,
			Self::Jmp(_, _) => {
				std::mem::size_of::<JmpMode>() as u32 * 8 + Address::BITS
			}
			Self::JmpIfFalse(_, _, _) | Self::JmpIfFalse(_, _, _) => {
				std::mem::size_of::<JmpMode>() as u32 * 8 + Address::BITS + Reg::BITS
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
			}
			| Self::Call(_, _, _) => 3 * Reg::BITS,
			Self::LoadF(_, _) => Reg::BITS + u16::BITS,
			Self::Clock(_) | Self::Pop(_) | Self::Push(_) | Self::LoadTrue(_) | Self::LoadFalse(_) => Reg::BITS,
			Self::LoadFloat(_, _) => Reg::BITS + 64, // f64
			// Self::GetArg(_, _) => Reg::BITS + u8::BITS,
			_ => 0
		};
		assert!(to_add % 8 == 0);
		n += to_add / 8;
		n as usize
	}
}
