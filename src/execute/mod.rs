#![allow(clippy::cast_precision_loss)]

use crate::parser::ast::Literal;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub mod stack_bytecode;
pub mod walker;
pub mod register_bytecode;

impl Add for Literal {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => Self::Int(x + y),
				Self::Float(y) => Self::Float(x as f64 + y),
				//Self::Bool(y) => Self::Int(x + y as i128),
				Self::String(y) => Self::String(x.to_string() + y.as_str()),
				Self::Bool(_) => panic!("Unknow operation")
			},
			Self::Bool(_) => panic!("cannot add bool"),
			Self::String(x) => match rhs {
				Self::String(y) => Self::String(x + y.as_str()),
				Self::Int(y) => Self::String(x + y.to_string().as_str()),
				Self::Float(y) => Self::String(x + y.to_string().as_str()),
				Self::Bool(y) => Self::String(x + y.to_string().as_str())
			},
			Self::Float(x) => match rhs {
				Self::Float(y) => Self::Float(x + y),
				Self::Int(y) => Self::Float(x + y as f64),
				Self::String(y) => Self::String(x.to_string() + y.as_str()),
				Self::Bool(_) => panic!("Unknow operation")
			}
		}
	}
}

impl Sub for Literal {
	type Output = Self;

	fn sub(self, rhs: Self) -> Self::Output {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => Self::Int(x - y),
				Self::Float(y) => Self::Float(x as f64 - y),
				_ => panic!("Unknow operation")
			},
			Self::Bool(_) => panic!("cannot sub bool"),
			Self::String(_) => panic!("cannot sub str"),
			Self::Float(x) => match rhs {
				Self::Float(y) => Self::Float(x - y),
				Self::Int(y) => Self::Float(x - y as f64),
				_ => panic!("Unknow operation")
			}
		}
	}
}

impl Neg for Literal {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self::Int(0) - self
	}
}

impl Mul for Literal {
	type Output = Self;
	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => Self::Int(x * y),
				Self::Float(y) => Self::Float(x as f64 * y),
				_ => panic!("tried to mul string or bool")
			},
			Self::Float(x) => match rhs {
				Self::Int(y) => Self::Float(x * y as f64),
				Self::Float(y) => Self::Float(x * y),
				_ => panic!("tried to mul string or bool")
			},
			_ => panic!("tried to mul string or bool")
		}
	}
}

impl Div for Literal {
	type Output = Self;
	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => Self::Int(x / y),
				Self::Float(y) => Self::Float(x as f64 / y),
				_ => panic!("tried to mul string or bool")
			},
			Self::Float(x) => match rhs {
				Self::Int(y) => Self::Float(x / y as f64),
				Self::Float(y) => Self::Float(x / y),
				_ => panic!("tried to mul string or bool")
			},
			_ => panic!("tried to mul string or bool")
		}
	}
}
