#![allow(clippy::pedantic)]

use crate::utils::stack::Stack;
use std::ptr::null;

use super::instructions::Reg;

pub const CALL_STACK_SIZE: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallStack<const N: usize> {
	pub stack: [CallFrame; N],
	pub top: usize
}

impl<const N: usize> Stack for CallStack<N> {
	type Value = CallFrame;

	fn append(&mut self, other: &[Self::Value]) {
		#[cfg(debug_assertions)]
		assert!(other.len() + self.len() <= N);
		let len = self.len();
		self.stack[len..len + other.len()].copy_from_slice(other);
	}

	fn push(&mut self, val: Self::Value) {
		#[cfg(debug_assertions)]
		assert!(self.top < N);
		unsafe {
			*self.stack.get_unchecked_mut(self.top) = val;
		}
		self.top += 1;
	}

	fn pop(&mut self) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(self.top > 0);
		self.top -= 1;
		unsafe { *self.stack.get_unchecked(self.top) }
	}

	fn get(&self, i: usize) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe { *self.stack.get_unchecked(i) }
	}

	fn get_mut(&mut self, i: usize) -> &mut Self::Value {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe { self.stack.get_unchecked_mut(i) }
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe {
			*self.stack.get_unchecked_mut(i) = val;
		}
	}

	fn last(&self) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(self.top > 0);
		unsafe { *self.stack.get_unchecked(self.top - 1) }
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		#[cfg(debug_assertions)]
		assert!(self.top > 0);
		unsafe { self.stack.get_unchecked_mut(self.top - 1) }
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
			stack: [CallFrame::empty(); N],
			top: 1
		}
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallFrame {
	pub base: *const u8,
	pub pc: *const u8,
	pub arg_count: u8,
	pub ret_count: u8,
	pub reg0_p: usize,
	pub ret_reg: Reg
}

impl CallFrame {
	pub const fn new(
		pc: *const u8,
		arg_count: u8,
		ret_count: u8,
		reg0_p: usize,
		ret_reg: u8
	) -> Self {
		Self {
			base: pc,
			pc,
			arg_count,
			ret_count,
			reg0_p,
			ret_reg
		}
	}

	pub const fn empty() -> Self {
		Self {
			base: null(),
			pc: null(),
			arg_count: 0,
			ret_count: 0,
			reg0_p: 0,
			ret_reg: 0
		}
	}

	#[inline]
	pub const unsafe fn pc(&self) -> usize {
		self.pc.offset_from(self.base) as usize
	}

	#[inline(always)]
	pub unsafe fn increment_pc(&mut self) {
		self.add_to_pc(1);
	}

	#[inline(always)]
	pub unsafe fn add_to_pc(&mut self, count: usize) {
		self.pc = self.pc.add(count);
	}

	#[inline(always)]
	pub unsafe fn remove_from_pc(&mut self, count: usize) {
		self.pc = self.pc.sub(count);
	}

	#[inline(always)]
	pub unsafe fn set_pc(&mut self, count: usize) {
		self.pc = self.base.add(count);
	}
}
