#![allow(clippy::module_name_repetitions)]
use super::{Address, Lit};
use crate::utils::stack::Stack;
use std::{
	cmp::Ordering,
	ops::{Add, Div, Mul, Sub}
};

const VM_STACK_DEFAULT_CAPACITY: usize = 2048;

#[derive(Debug)]
pub struct VmStack {
	stack: Vec<StackValue>
}

impl Stack for VmStack {
	// TODO: do i really need a Stack trait since i don't use slices ?
	type Value = StackValue;

	fn append(&mut self, other: &[Self::Value]) {
		self.stack.append(&mut other.to_owned());
	}

	fn push(&mut self, val: Self::Value) {
		self.stack.push(val);
	}

	fn pop(&mut self) -> Self::Value {
		self.stack.pop().unwrap()
	}

	fn get(&self, i: usize) -> Self::Value {
		*self.stack.get(i).unwrap()
	}

	fn get_mut(&mut self, i: usize) -> &mut Self::Value {
		self.stack.get_mut(i).unwrap()
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		*self.stack.get_mut(i).unwrap() = val; // maybe use insert
	}

	fn last(&self) -> Self::Value {
		*self.stack.last().unwrap()
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		self.stack.last_mut().unwrap()
	}

	#[inline]
	fn len(&self) -> usize {
		self.stack.len()
	}

	fn remove(&mut self, n: usize) {
		self.stack.truncate(self.len() - n);
	}

	fn reset(&mut self) {
		self.stack.clear();
	}
}

impl VmStack {
	pub fn new() -> Self {
		Self {
			stack: Vec::new()
		}
	}

	pub fn preallocate(&mut self, n: usize) {
		self.stack.reserve(n);
	}
	
	// n included
	pub fn preallocate_up_to(&mut self, n: usize) {
		let n = n + 1;
		if n > self.stack.capacity() {
			self.stack.reserve(n - self.stack.capacity());
		} 
	}

	// n included
	pub fn preset_up_to(&mut self, n: usize) {
		let n = n + 1;
		if n > self.len() {
			self.stack.resize_with(n, Default::default)
		}
	}
}

impl Default for VmStack {
	fn default() -> Self {
		Self {
			stack: Vec::with_capacity(VM_STACK_DEFAULT_CAPACITY)
		}
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

impl Default for StackValue {
	fn default() -> Self {
		Self::zero()
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
