#![allow(clippy::module_name_repetitions)]

use std::{
	cmp::Ordering,
	ops::{Add, Div, Mul, Sub}
};

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
				Self::Float(y) => Self::Float(x as f64 + y)
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
				Self::Float(y) => Self::Float(x as f64 - y)
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => Self::Float(x - y),
				Self::Int(y) => Self::Float(x - y as f64)
			}
		}
	}
}

impl Mul for StackValue {
	type Output = Self;

	fn mul(self, rhs: Self) -> Self::Output {
		match self {
			Self::Bool(_) => panic!("Can't add bools (lhs)"),
			Self::Int(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Int(y) => Self::Int(x * y),
				Self::Float(y) => Self::Float(x as f64 * y)
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => Self::Float(x * y),
				Self::Int(y) => Self::Float(x * y as f64)
			}
		}
	}
}

impl Div for StackValue {
	type Output = Self;

	fn div(self, rhs: Self) -> Self::Output {
		match self {
			Self::Bool(_) => panic!("Can't add bools (lhs)"),
			Self::Int(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Int(y) => Self::Float(x as f64 / y as f64),
				Self::Float(y) => Self::Float(x as f64 / y)
			},
			Self::Float(x) => match rhs {
				Self::Bool(_) => panic!("Can't add bools (rhs)"),
				Self::Float(y) => Self::Float(x / y),
				Self::Int(y) => Self::Float(x / y as f64)
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
				Self::Float(y) => cmp(x as f64, *y)
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


/*
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum T {
	Int,
	Float,
	Bool
}

#[derive(Clone, Copy)]
pub union StackValueUnion {
	pub int: i128,
	pub float: f64,
	pub bool: bool
}

#[derive(Clone, Copy)]
pub struct StackValue {
	pub ty: T,
	pub val: StackValueUnion
}

macro_rules! stack_op {
	($trait:ident, $name:ident, $op:tt, $cond:literal) => {
		impl $trait for StackValue {
			type Output = Self;

			fn $name(self, rhs: Self) -> Self::Output {
				assert!(!(self.ty == T::Bool || rhs.ty == T::Bool), "Can't operate bools");
				if self.ty == rhs.ty {

					let val = if $cond && self.ty == T::Int {
						unsafe {
							StackValueUnion { float: self.val.int as f64 $op rhs.val.int as f64 }
						}
					} else {
						unsafe {
							match self.ty {
								T::Float => StackValueUnion { float: self.val.float $op rhs.val.float },
								T::Int => StackValueUnion { int: self.val.int $op rhs.val.int },
								T::Bool => unreachable!()
							}
						}
					};

					StackValue {
						ty: if $cond { T::Float } else { self.ty },
						val
					}
				} else {
					StackValue {
						ty: T::Float,
						val: StackValueUnion {
							float: unsafe {
								match self.ty {
									T::Float => self.val.float $op rhs.val.int as f64,
									T::Int => self.val.int as f64 $op rhs.val.float,
									T::Bool => unreachable!()
								}
							}
						}
					}
				}
			}
		}
	}
}

stack_op!(Add, add, +, false);
stack_op!(Sub, sub, -, false);
stack_op!(Mul, mul, *, false);
stack_op!(Div, div, /, true);


impl StackValue {
	pub const fn zero() -> Self {
		Self { ty: T::Int, val: StackValueUnion {int: 0} }
	}

	pub const fn Int(val: i128) -> Self {
		Self { ty: T::Int, val: StackValueUnion { int: val } }
	}

	pub const fn Float(val: f64) -> Self {
		Self { ty: T::Float, val: StackValueUnion { float: val } }
	}

	pub const fn Bool(val: bool) -> Self {
		Self { ty: T::Bool, val: StackValueUnion { bool: val } }
	}
}


impl StackValue {
	pub fn cmp(self, rhs: &Self) -> Ordering {
		assert!(!(self.ty == T::Bool || rhs.ty == T::Bool), "Can't add bools");
		if self.ty == rhs.ty {
			match self.ty {
				T::Float => unsafe {cmp(self.val.float, rhs.val.float) },
				T::Int => unsafe {self.val.int.cmp(&rhs.val.int) },
				T::Bool => unreachable!()
			}
		} else {
			match self.ty {
				T::Float => unsafe {cmp(self.val.float, rhs.val.int as f64) },
				T::Int => unsafe {cmp(self.val.int as f64, rhs.val.float) },
				T::Bool => unreachable!()
			}
		}
	}
}


#[cfg(test)]
mod tests {
	use pretty_assertions::assert_eq;
	use std::cmp::Ordering;
	use crate::execute::register_bytecode::T;

use super::StackValue;

	#[test]
	fn cmp() {
		let a = StackValue::Int(1462);
		let b = StackValue::Int(1000);
		assert_eq!(a.cmp(b), Ordering::Greater);
	}

	#[test]
	fn sub() {
		let a = StackValue::Int(1462);
		let b = StackValue::Int(1000);
		
		let c = a - b;


		assert_eq!(unsafe {c.val.int}, 462);
		assert_eq!(c.ty, T::Int);


	}
}
*/