#![allow(clippy::module_name_repetitions)]
use super::Lit;
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

	fn get(&self, i: usize) -> &Self::Value {
		self.stack.get(i).unwrap()
	}

	fn get_mut(&mut self, i: usize) -> &mut Self::Value {
		self.stack.get_mut(i).unwrap()
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		*self.stack.get_mut(i).unwrap() = val; // maybe use insert
	}

	fn last(&self) -> &Self::Value {
		self.stack.last().unwrap()
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
		Self { stack: Vec::new() }
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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum StackValue {
	Int(Lit),
	Float(f64),
	Bool(bool),
	Function(u16),
	String(String)
	// TODO: type
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


macro_rules! stack_op_ref {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		impl $trait<&StackValue> for StackValue {
			type Output = Self;

			fn $name(self, rhs: &Self) -> Self::Output {
				match self {
					Self::Int(x) => match rhs {
						Self::Int(y) => if $all_floats { Self::Float(x as f64 $op *y as f64) } else { Self::Int(x $op y) },
						Self::Float(y) => Self::Float(x as f64 $op y),
						_ => unreachable!()
					},
					Self::Float(x) => match rhs {
						Self::Float(y) => Self::Float(x $op y),
						Self::Int(y) => Self::Float(x $op *y as f64),
						_ => unreachable!()
					},
					_ => unreachable!()
				}
			}
		}
	};
}

macro_rules! stack_op_ref_ {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		impl $trait<StackValue> for &StackValue {
			type Output = StackValue;

			fn $name(self, rhs: StackValue) -> Self::Output {
				match self {
					StackValue::Int(x) => match rhs {
						StackValue::Int(y) => if $all_floats { StackValue::Float(*x as f64 $op y as f64) } else { StackValue::Int(x $op y) },
						StackValue::Float(y) => StackValue::Float(*x as f64 $op y),
						_ => unreachable!()
					},
					StackValue::Float(x) => match rhs {
						StackValue::Float(y) => StackValue::Float(x $op y),
						StackValue::Int(y) => StackValue::Float(x $op y as f64),
						_ => unreachable!()
					},
					_ => unreachable!()
				}
			}
		}
	};
}

macro_rules! gen_stack_op {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		stack_op!($trait, $name, $op, $all_floats);
		stack_op_ref!($trait, $name, $op, $all_floats);
		stack_op_ref_!($trait, $name, $op, $all_floats);
	};
}

// FIXME: handle overflows
gen_stack_op!(Add, add, +, false);
gen_stack_op!(Sub, sub, -, false);
gen_stack_op!(Mul, mul, *, false);
gen_stack_op!(Div, div, /, true);


impl StackValue {
	#[allow(clippy::should_implement_trait)]
	pub fn cmp(&self, rhs: &Self) -> Ordering {
		match self {
			Self::Int(x) => match rhs {
				Self::Int(y) => x.cmp(y),
				Self::Float(y) => cmp(*x as f64, *y),
				_ => unreachable!()
			},
			Self::Float(x) => match rhs {
				Self::Float(y) => cmp(*x, *y),
				Self::Int(y) => cmp(*x, *y as f64),
				_ => unreachable!()
			},
			_ => unreachable!()
		}
	}

	pub const fn zero() -> Self {
		Self::Int(0)
	}

	pub fn is_true(&self) -> bool {
		self == &Self::Bool(true)
	}

	pub fn is_false(&self) -> bool {
		!self.is_true()
	}

	pub fn as_string(&self) -> &String {
		let Self::String(res) = self else {
			panic!("Expected string when extracting StackValue")
		};
		res
	}
	
	pub fn as_int(&self) -> Lit {
		let Self::Int(res) = self else {
			panic!("Expected int when extracting StackValue")
		};
		*res
	}
	
	pub fn as_float(&self) -> f64 {
		let Self::Float(res) = self else {
			panic!("Expected float when extracting StackValue")
		};
		*res
	}
	
	pub fn as_bool(&self) -> bool {
		let Self::Bool(res) = self else {
			panic!("Expected bool when extracting StackValue")
		};
		*res
	}
	
	pub fn as_fn(&self) -> u16 {
		let Self::Function(res) = self else {
			panic!("Expected function when extracting StackValue")
		};
		*res
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

#[cfg(test)]
mod tests {
	use super::{StackValue, VmStack};
	use crate::utils::stack::Stack;
	use pretty_assertions::assert_eq;

	#[test]
	fn vm_stack() {
		let mut stack = VmStack::new();

		assert_eq!(stack.len(), 0);

		let dummy = StackValue::zero();
		stack.push(dummy);
		assert_eq!(stack.len(), 1);

		let vec = (1..=10).collect::<Vec<_>>();
		assert_eq!(vec, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);

		let vec = vec
			.into_iter()
			.map(|x| StackValue::Int(x as i64))
			.collect::<Vec<StackValue>>();

		stack.append(&vec);
		assert_eq!(stack.len(), 11);

		stack.preallocate_up_to(10);
		assert_eq!(stack.len(), 11);
		assert!(stack.stack.capacity() >= 11);

		stack.preallocate_up_to(15);
		assert_eq!(stack.len(), 11);
		assert!(stack.stack.capacity() >= 16);

		assert_eq!(stack.pop(), StackValue::Int(10));
		assert_eq!(stack.len(), 10);

		stack.remove(2);
		assert_eq!(stack.len(), 8);

		assert_eq!(stack.get(3), &StackValue::Int(3));

		*stack.get_mut(3) = StackValue::Int(9);
		assert_eq!(stack.get(3), &StackValue::Int(9));
	}

	macro_rules! test_op {
		($op:tt; $t_a:ident, $a:literal; $t_b:ident, $b:literal; $t_e:ident, $e:literal) => {
			let a = StackValue::$t_a($a);
			let b = StackValue::$t_b($b);

			let expected = StackValue::$t_e($e);

			pretty_assertions::assert_eq!(a $op b, expected);
		};
	}

	macro_rules! test_cmp {
		($t_a:ident, $a:literal; $t_b:ident, $b:literal; $e:ident) => {
			let a = StackValue::$t_a($a);
			let b = StackValue::$t_b($b);

			let expected = std::cmp::Ordering::$e;

			pretty_assertions::assert_eq!(a.cmp(&b), expected);
		};
	}

	#[test]
	fn stack_value_zero() {
		assert_eq!(StackValue::zero(), StackValue::Int(0));
	}

	#[test]
	fn stack_value_op() {
		test_op!(+; Int, 10; Int, 20; Int, 30);
		test_op!(+; Int, 500; Int, -20; Int, 480);
		test_op!(+; Int, 0; Int, 0; Int, 0);
		test_op!(+; Float, 10.5; Float, 20.5; Float, 31.);
		test_op!(+; Float, 0.; Float, 0.; Float, 0.);
		test_op!(+; Float, 10.; Int, 20; Float, 30.);

		test_op!(-; Int, 10; Int, 20; Int, -10);
		test_op!(-; Int, 500; Int, -20; Int, 520);
		test_op!(-; Int, 0; Int, 0; Int, 0);
		test_op!(-; Float, 10.5; Float, 20.5; Float, -10.);
		test_op!(-; Float, 0.; Float, 0.; Float, 0.);
		test_op!(-; Float, 10.; Int, 20; Float, -10.);

		test_op!(*; Int, 10; Int, 20; Int, 200);
		test_op!(*; Int, 500; Int, -20; Int, -10_000);
		test_op!(*; Int, 0; Int, 0; Int, 0);
		test_op!(*; Float, 10.5; Float, 20.5; Float, 215.25);
		test_op!(*; Float, 0.; Float, 0.; Float, 0.);
		test_op!(*; Float, 10.; Int, 20; Float, 200.);

		test_op!(/; Int, 10; Int, 20; Float, 0.5);
		test_op!(/; Int, 500; Int, -20; Float, -25.);
		test_op!(/; Int, 0; Int, 1; Float, 0.);
		test_op!(/; Float, 10.5; Float, 20.; Float, 0.525);
		test_op!(/; Float, 0.; Float, 1.; Float, 0.);
		test_op!(/; Float, 10.; Int, 20; Float, 0.5);
	}

	#[test]
	fn stack_value_cmp() {
		test_cmp!(Int, 10; Int, 10; Equal);
		test_cmp!(Int, -10; Int, 10; Less);
		test_cmp!(Int, 10; Int, -10; Greater);
		test_cmp!(Int, 0; Int, 0; Equal);
		test_cmp!(Int, 10; Float, 10.; Equal);
		test_cmp!(Float, 10.; Int, 10; Equal);
		test_cmp!(Int, 10; Float, -10.; Greater);
		test_cmp!(Float, 10.; Int, -10; Greater);
		test_cmp!(Float, 10.5; Float, 10.5; Equal);
		test_cmp!(Float, 10.; Float, 10.5; Less);
		test_cmp!(Float, -10.; Float, 10.5; Less);
		test_cmp!(Float, 25.7; Float, 10.; Greater);
	}

	#[test]
	#[should_panic]
	fn stack_value_op_bool() {
		let _result = StackValue::Bool(true) + StackValue::Bool(true);
	}

	#[test]
	#[should_panic]
	fn stack_value_op_function() {
		let _result = StackValue::Function(0) + StackValue::Function(1);
	}
}
