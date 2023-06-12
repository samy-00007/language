#![allow(clippy::pedantic)]

use crate::utils::stack::Stack;
use std::ptr::null_mut;

use super::Reg;

pub const CALL_STACK_SIZE: usize = 256;
pub const REGISTER_COUNT: usize = 256;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallStack<const N: usize> {
	pub stack: [CallFrame; N],
	pub top: *mut CallFrame,
	pub base: *const CallFrame
}

impl<const N: usize> Stack for CallStack<N> {
	type Value = CallFrame;

	fn append(&mut self, other: &[Self::Value]) {
		#[cfg(debug_assertions)]
		assert!(other.len() + self.len() <= N);
		let len = self.len();
		self.stack[len..len+other.len()].copy_from_slice(other);
	}
	
	fn push(&mut self, val: Self::Value) {
		unsafe {
			self.assert_no_overflow();

			*(self.top) = val;
			self.top = self.top.add(1);
		}
	}
	
	fn pop(&mut self) -> Self::Value {
		unsafe {
			self.assert_no_underflow();

			self.top = self.top.sub(1);
			*self.top
		}
	}

	fn get(&self, i: usize) -> Self::Value {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe {
			self.assert_no_underflow();
			*self.base.add(i)
		}
	}
	
	fn get_mut(&mut self, i: usize) -> &mut Self::Value {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe {
			self.base.add(i).cast_mut().as_mut().unwrap() // TODO: maybe just return a pointer
		}
	}
	
	fn set(&mut self, i: usize, val: Self::Value) {
		#[cfg(debug_assertions)]
		assert!(i < N);
		unsafe {
			*self.base.add(i).cast_mut() = val;
		}
	}

	fn last(&self) -> Self::Value {
		unsafe {
			*self.top.sub(1)
		}
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		unsafe {
			self.top.sub(1).as_mut().unwrap_unchecked() // TODO: maybe just return the pointer
		}
	}

	fn len(&self) -> usize {
		unsafe {
			self.top.offset_from(self.base) as usize
		}
	}

	fn remove(&mut self, n: usize) {
		self.top = unsafe { self.top.sub(n) };
		unsafe {
			#[cfg(debug_assertions)]
			assert!(self.base.offset_from(self.top) >= 0);
		}
	}

	fn reset(&mut self) {
		self.top = self.base.cast_mut();
	}
}

impl<const N: usize> CallStack<N> {
	#[inline(always)] 
	pub const fn new() -> Self {
		Self { stack: [CallFrame::empty(); N], top: null_mut(), base: null_mut() }
	}

	/// SAFETY: caller MUST call this function after initializing the call stack
	/// otherwise, it will produces UB
	/// note: depending on how deep this function in run, it may or not UB
	/// (depending on whether or not the call stack is moved from a function stack by a return for instance)
	pub fn init_pointers(&mut self, n: usize) {
		self.top = unsafe { self.stack.as_mut_ptr().add(n) };
		self.base = self.stack.as_ptr();
	}

	#[cfg(debug_assertions)]
	unsafe fn assert_no_overflow(&self) {
		assert!(self.base.offset_from(self.top) < N as isize);
	}

	#[cfg(debug_assertions)]
	unsafe fn assert_no_underflow(&self) {
		assert!(self.top.offset_from(self.base) > 0);
	}

	#[cfg(not(debug_assertions))]
	unsafe fn assert_no_overflow(&self) {}

	#[cfg(not(debug_assertions))]
	unsafe fn assert_no_underflow(&self) {}

}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallFrame {
	pub pc: usize,
	pub arg_count: usize,
	pub reg0_p: usize,
	pub ret_reg: Reg
}

impl CallFrame {
	pub const fn new(pc: usize, arg_count: usize, reg0_p: usize, ret_reg: u8) -> Self {
		Self { pc, arg_count, reg0_p, ret_reg }
	}
	
	pub const fn empty() -> Self {
		Self { pc: 0, arg_count: 0, reg0_p: 0, ret_reg: 0 }
	}
}


#[cfg(test)]
mod tests {
    use crate::utils::stack::Stack;
	use pretty_assertions::assert_eq;

    use super::{CallStack, CallFrame};

	#[test]
	fn callstack_append() {
		let mut a = CallStack::<10>::new();
		a.init_pointers(0);
		let b = [CallFrame::new(1, 1, 1, 1); 5];
		a.append(&b);
		
		let mut expected = CallStack::<10>::new();
		expected.init_pointers(0);
		unsafe {println!("exp: {:?}", *expected.top)}
		for _ in 0..5 {
			expected.push(CallFrame::new(1, 1, 1, 1));
		}

		// assert_eq!(a, expected);
	}
}
