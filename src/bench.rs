#![allow(clippy::cast_precision_loss)]

extern crate test;
use test::Bencher;
use std::ops::Add;
use rand::prelude::*;


#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum StackValue {
	Int(i64),
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


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum T {
	Int,
	Float,
	Bool
}

#[derive(Clone, Copy)]
union StackValueUnion {
	int: i64,
	float: f64,
	bool: bool
}

#[derive(Clone, Copy)]
struct StackValueStruct {
	ty: T,
	val: StackValueUnion
}

impl Add for StackValueStruct {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		assert!(!(self.ty == T::Bool || rhs.ty == T::Bool), "Can't add bools");
		
		if self.ty == rhs.ty {
			Self {
				ty: self.ty,
				val: unsafe {
					match self.ty {
						T::Float => StackValueUnion { float: self.val.float + rhs.val.float },
						T::Int => StackValueUnion { int: self.val.int + rhs.val.int },
						T::Bool => unreachable!()
					}
				}
			}
		} else {
			Self {
				ty: T::Float,
				val: StackValueUnion {
					float: unsafe {
					match self.ty {
						T::Float => self.val.float + rhs.val.int as f64,
						T::Int => self.val.int as f64 + rhs.val.float,
						T::Bool => unreachable!()
					}
				}
			}
			}
			
		}
	}
}

#[bench]
fn bench_enum_int(be: &mut Bencher) {
	be.iter(|| {
		let a = StackValue::Int(random::<i64>());
		let b = StackValue::Int(random::<i64>());
		// test::black_box(a + b)
		a + b
	});
}

#[bench]
fn bench_enum_float(be: &mut Bencher) {
	be.iter(|| {
		let a = StackValue::Float(random::<f64>());
		let b = StackValue::Float(random::<f64>());
		// test::black_box(a + b)
		a + b
	});
}

#[bench]
fn bench_union_int(be: &mut Bencher) {
	be.iter(|| {
		let a = StackValueStruct { ty: T::Int, val: StackValueUnion { int: random::<i64>() } };
		let b = StackValueStruct { ty: T::Int, val: StackValueUnion { int: random::<i64>() } };
		// test::black_box(a + b)
		a + b
	});
}

#[bench]
fn bench_union_float(be: &mut Bencher) {
	be.iter(|| {
		let a = StackValueStruct { ty: T::Float, val: StackValueUnion { float: random::<f64>() } };
		let b = StackValueStruct { ty: T::Float, val: StackValueUnion { float: random::<f64>() } };
		// test::black_box(a + b)
		a + b
	});
}

