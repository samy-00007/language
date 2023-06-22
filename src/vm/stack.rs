#![allow(clippy::module_name_repetitions)]
use super::{Address, Lit};
use crate::utils::stack::Stack;
use std::{
	cmp::Ordering,
	ops::{Add, Div, Mul, Sub}
};

#[derive(Debug)]
pub struct VmStack {
	stack: Vec<StackValue>,
	pub top: usize
}

impl Stack for VmStack {
	// TODO: do i really need a Stack trait since i don't use slices ?
	type Value = StackValue;

	fn append(&mut self, other: &[Self::Value]) {
		self.top += other.len();
		self.stack.append(&mut other.to_vec());
	}

	fn push(&mut self, val: Self::Value) {
		self.top += 1;
		self.stack.push(val);
	}

	fn pop(&mut self) -> Self::Value {
		self.top -= 1;
		self.stack.pop().unwrap()
	}

	fn get(&self, i: usize) -> Self::Value {
		*self.stack.get(i).unwrap()
	}

	fn get_mut(&mut self, i: usize) -> &mut Self::Value {
		self.stack.get_mut(i).unwrap()
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		*self.stack.get_mut(i).unwrap() = val;
	}

	fn last(&self) -> Self::Value {
		*self.stack.last().unwrap()
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		self.stack.last_mut().unwrap()
	}

	fn len(&self) -> usize {
		//self.stack.len()
		self.top
	}

	fn remove(&mut self, n: usize) {
		//self.stack.truncate(self.len() - n); // TODO: fix that
		self.top -= n;
	}

	fn reset(&mut self) {
		self.stack.clear();
		self.top = 0;
	}
}

impl VmStack {
	pub const fn new() -> Self {
		Self {
			stack: Vec::new(),
			top: 0
		}
	}

	pub fn preallocate(&mut self, other: &[StackValue]) {
		self.stack.extend_from_slice(other);
	}

	pub fn capacity(&self) -> usize {
		self.stack.len()
	}
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum StackValue {
	Int(Lit),
	Float(f64),
	Bool(bool),
	Function(*const u8) // TODO: type
}

macro_rules! stack_op {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		impl $trait for StackValue {
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
// FIXME: handle overflows
stack_op!(Add, add, +, false);
stack_op!(Sub, sub, -, false);
stack_op!(Mul, mul, *, false);
stack_op!(Div, div, /, true);

impl StackValue {
	pub fn cmp(self, rhs: &Self) -> Ordering {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => x.cmp(y),
				Self::Float(y) => cmp(x as f64, *y),
				_ => unreachable!()
			},
			Self::Float(x) => match rhs {
				Self::Float(y) => cmp(x, *y),
				Self::Int(y) => cmp(x, *y as f64),
				_ => unreachable!()
			},
			_ => unreachable!()
		}
	}

	pub const fn zero() -> Self {
		Self::Int(0)
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
