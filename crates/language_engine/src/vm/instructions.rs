use std::fmt::Display;

pub use super::stack::*;

// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

// https://github.com/boa-dev/boa/blob/main/boa_engine/src/vm/opcode/mod.rs

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
	Lt,
	Addl,
	Subl,
	Mull,
	Divl,
	Ltl,
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
	Add { op_1: Reg, op_2: Reg, dst: Reg },
	Sub { op_1: Reg, op_2: Reg, dst: Reg },
	Mul { op_1: Reg, op_2: Reg, dst: Reg },
	Div { op_1: Reg, op_2: Reg, dst: Reg },
	Lt { op_1: Reg, op_2: Reg, dst: Reg },
	Addl { op_1: Reg, op_2: Lit, dst: Reg },
	Subl { op_1: Reg, op_2: Lit, dst: Reg },
	Mull { op_1: Reg, op_2: Lit, dst: Reg },
	Divl { op_1: Reg, op_2: Lit, dst: Reg },
	Ltl { op_1: Reg, op_2: Lit, dst: Reg },
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

macro_rules! add_n {
	($n:ident, $t:ty) => {
		#[allow(dead_code)]
		fn $n(buffer: &mut Vec<u8>, n: $t) {
			buffer.extend(n.to_le_bytes());
		}
	};
}

add_n!(add_u8, u8);
add_n!(add_u16, u16);
add_n!(add_u32, u32);
add_n!(add_u64, u64);

add_n!(add_i8, i8);
add_n!(add_i16, i16);
add_n!(add_i32, i32);
add_n!(add_i64, i64);

add_n!(add_f64, f64);

macro_rules! match_ops {
	[
		$($name_1:ident);* ;;
		$($name_2:ident; $(($a:ident, $fn:ident, $t:ty)),*);* ;;
		$($name_3:ident; $(($a_:ident, $fn_:ident, $t_:ty)),*);*
		] => {
		pub fn compile(self, buffer: &mut Vec<u8>) {
			match self {
				$(
					Self::$name_1 => {
						add_u8(buffer, Opcode::$name_1 as u8);
					}
				),*

				$(
					Self::$name_2 ( $($a),* ) => {
						add_u8(buffer, Opcode::$name_2 as u8);
						$(
							$fn(buffer, $a as $t);
						)*
					}
				),*
				$(
					Self::$name_3 { $($a_),* } => {
						add_u8(buffer, Opcode::$name_3 as u8);
						$(
							$fn_(buffer, $a_ as $t_);
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
		Add; (op_1, add_u8, Reg), (op_2, add_u8, Reg), (dst, add_u8, Reg);
		Sub; (op_1, add_u8, Reg), (op_2, add_u8, Reg), (dst, add_u8, Reg);
		Mul; (op_1, add_u8, Reg), (op_2, add_u8, Reg), (dst, add_u8, Reg);
		Div; (op_1, add_u8, Reg), (op_2, add_u8, Reg), (dst, add_u8, Reg);
		Lt; (op_1, add_u8, Reg), (op_2, add_u8, Reg), (dst, add_u8, Reg);
		Addl; (op_1, add_u8, Reg), (op_2, add_i64, Lit), (dst, add_u8, Reg);
		Subl; (op_1, add_u8, Reg), (op_2, add_i64, Lit), (dst, add_u8, Reg);
		Mull; (op_1, add_u8, Reg), (op_2, add_i64, Lit), (dst, add_u8, Reg);
		Divl; (op_1, add_u8, Reg), (op_2, add_i64, Lit), (dst, add_u8, Reg);
		Ltl; (op_1, add_u8, Reg), (op_2, add_i64, Lit), (dst, add_u8, Reg)
	];
}
