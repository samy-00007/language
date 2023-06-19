use crate::parser::ast::{Ty, Literal};
use std::ops::{Add, Sub, Mul, Div};


#[derive(Debug)]
pub struct Var {
	pub reg: u8,
	pub ty: Type
}

impl Var {
	pub const fn new(reg: u8, ty: Type) -> Self {
		Self { reg, ty }
	}
}

#[derive(Debug)]
pub enum Type {
	Bool,
	Number,
	String
}

impl From<Ty> for Type {
	fn from(value: Ty) -> Self {
		let Ty::Ident(ty) = value else {unreachable!()}; // TODO: handle that
		match ty.as_str() {
				"string" => Self::String,
				"number" => Self::Number,
				"bool" => Self::Bool,
				_ => unreachable!()
			}
	}
}

#[macro_export]
macro_rules! match_infix_op {
	($op:ident, $reg_1:ident, $reg_2:ident, $dst:ident; $($name:ident),*) => {
		match $op {
			$(
				Operator::$name => Instr::$name {
					op_1: $reg_1,
					op_2: $reg_2,
					dst: $dst,
				},
			)*
			x => todo!("Operator {x:?} not yet handled")
		}
	};
}

#[macro_export]
macro_rules! match_infix_op_lit {
	($op:ident, $reg_1:ident, $reg_2:ident, $dst:ident; $(($op_name:ident, $name:ident)),*) => {
		match $op {
			$(
				Operator::$op_name => Instr::$name {
					op_1: $reg_1,
					op_2: $reg_2,
					dst: $dst,
				},
			)*
			x => todo!("Operator {x:?} not yet handled")
		}
	};
}

macro_rules! literal_op {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		impl $trait for Literal {
			type Output = Self;

			fn $name(self, rhs: Self) -> Self::Output {
				match self {
					Self::Int(x) => match rhs {
						Self::Int(y) => if $all_floats { Self::Float(x as f64 $op y as f64) } else { Self::Int(x $op y) },
						Self::Float(y) => Self::Float(x as f64 $op y),
						_ => unreachable!()
					},
					Self::Float(x) => match rhs {
						Self::Float(y) => Self::Float(x $op y),
						Self::Int(y) => Self::Float(x $op y as f64),
						_ => unreachable!()
					},
					_ => unreachable!()
				}
			}
		}
	};
}

literal_op!(Add, add, +, false);
literal_op!(Sub, sub, -, false);
literal_op!(Mul, mul, -, false);
literal_op!(Div, div, /, true);