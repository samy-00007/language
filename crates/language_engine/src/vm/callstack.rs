#![allow(clippy::pedantic)]

use crate::utils::stack::Stack;
use std::cell::RefCell;

use super::{opcodes::Reg, program::Program};

pub const CALL_STACK_SIZE: usize = 256;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallStack<const N: usize> {
	pub stack: [RefCell<CallFrame>; N],
	pub top: usize
}

impl<const N: usize> Stack for CallStack<N> {
	type Value = RefCell<CallFrame>;

	fn append(&mut self, _other: &[Self::Value]) {
		unimplemented!()
	}

	fn push(&mut self, val: Self::Value) {
		#[cfg(debug_assertions)]
		assert!(self.top < N);
		self.stack[self.top] = val;
		self.top += 1;
	}

	fn pop(&mut self) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(self.top > 0);
		self.top -= 1;
		self.stack[self.top].clone()
	}

	fn get(&self, i: usize) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(i < self.top);
		self.stack[i].clone()
	}

	fn get_mut(&mut self, _i: usize) -> &mut Self::Value {
		// we are using refcells, no need for that
		unimplemented!()
	}

	fn set(&mut self, _i: usize, _val: Self::Value) {
		// same as for `get_mut`
		unimplemented!()
	}

	fn last(&self) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(self.top > 0);
		self.stack[self.top - 1].clone()
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		//same as for `get_mut` and `set`
		unimplemented!()
	}

	fn len(&self) -> usize {
		self.top
	}

	fn remove(&mut self, n: usize) {
		assert!(self.top > n);
		self.top -= n;
	}

	fn reset(&mut self) {
		self.top = 0;
	}
}

impl<const N: usize> Default for CallStack<N> {
	fn default() -> Self {
		Self {
			stack: std::array::from_fn(|_| RefCell::new(CallFrame::empty())),
			top: 1
		}
	}
}

impl<const N: usize> CallStack<N> {
	pub fn new() -> Self {
		Self {
			stack: std::array::from_fn(|_| RefCell::new(CallFrame::empty())),
			top: 0
		}
	}
}

// TODO: maybe impl a Readable trait to standard nums
macro_rules! read_bytes {
	($name:ident, $t:tt) => {
		#[allow(dead_code)]
		pub fn $name(&mut self) -> $t {
			let size = std::mem::size_of::<$t>();
			let bytes = &self.function.code[self.pc..(self.pc + size)];
			self.pc += size;
			$t::from_le_bytes(bytes.try_into().unwrap())
		}
	};
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallFrame {
	function: Program,
	pc: usize,
	arg_count: u8,
	ret_count: u8,
	pub reg0_p: usize,
	pub ret_reg: Reg
}

impl CallFrame {
	pub const fn new(
		function: Program,
		pc: usize,
		arg_count: u8,
		ret_count: u8,
		reg0_p: usize,
		ret_reg: u8
	) -> Self {
		Self {
			function,
			pc,
			arg_count,
			ret_count,
			reg0_p,
			ret_reg
		}
	}

	pub const fn empty() -> Self {
		Self {
			function: Program::new(),
			pc: 0,
			arg_count: 0,
			ret_count: 0,
			reg0_p: 0,
			ret_reg: 0
		}
	}

	#[inline]
	pub fn _pc(&self) -> usize {
		self.pc
	}

	#[inline(always)]
	pub fn increment_pc(&mut self) {
		#[cfg(debug_assertions)]
		assert!(self.pc < usize::MAX);

		self.pc += 1;
	}

	#[inline(always)]
	pub fn set_pc(&mut self, count: usize) {
		self.pc = count;
	}

	pub fn read_u8(&mut self) -> u8 {
		let val = self.function.code[self.pc];
		self.increment_pc();
		val
	}

	#[allow(dead_code)]
	pub fn ensure_no_overlow(&self) {
		assert!(self.pc < self.function.code.len())
	}

	read_bytes!(read_u16, u16);
	read_bytes!(read_i16, i16);

	read_bytes!(read_u32, u32);
	read_bytes!(read_i32, i32);

	read_bytes!(read_u64, u64);
	read_bytes!(read_i64, i64);

	read_bytes!(read_f64, f64);
}

#[cfg(test)]
mod tests {
	use crate::vm::program::Program;

	use super::CallFrame;

	#[test]
	fn read_n() {
		let mut program = Program::new();
		program.code = vec![0; 20];
		program.code[0] = 1;

		let mut frame = CallFrame::new(program, 0, 0, 0, 0, 0);
		assert_eq!(frame.read_u8(), 1);
	}
}
