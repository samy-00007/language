use crate::utils::stack::Stack;

use super::vm::Register;

pub const CALL_STACK_SIZE: usize = 256;
pub const REGISTER_COUNT: usize = 256;

pub struct CallStack<const N: usize, const B: usize> {
	stack: [CallFrame<B>; N],
	pc: usize
}

impl<const N: usize, const B: usize> Stack for CallStack<N, B> {
	type Value = CallFrame<B>;

	fn push(&mut self, val: Self::Value) {
		assert!(self.pc < N - 1);
		self.stack[self.pc] = val;
		self.pc += 1;
	}

	fn pop(&mut self) -> Self::Value {
		assert!(self.pc > 0);
		self.pc -= 1;
		self.stack[self.pc]
	}

	fn get(&mut self, i: usize) -> Self::Value {
		assert!(i < N);
		*self.stack.get(i).unwrap()
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		assert!(i < N);
		*self.stack.get_mut(i).unwrap() = val;
	}

	fn last(&self) -> Self::Value {
		self.stack[self.pc - 1]
	}

	fn last_mut(&mut self) -> &mut Self::Value {
		self.stack.get_mut(self.pc - 1).unwrap()
	}

	fn len(&self) -> usize {
		self.pc
	}

	fn reset(&mut self) {
		self.pc = 0;
	}
}

impl<const N: usize, const B: usize> CallStack<N, B> {
	pub const fn new() -> Self {
		Self { stack: [CallFrame::<B>::empty(); N], pc: 1 }
	}
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CallFrame<const N: usize> {
	pub ret_pc: usize,
	pub arg_count: usize,
	pub arg0_i: usize,
	pub registers: [Register; N]
}

impl<const N: usize> CallFrame<N> {
	pub const fn new(ret_pc: usize, arg_count: usize, arg0_i: usize) -> Self {
		Self { ret_pc, arg_count, arg0_i, registers: [Register::Int(0); N] }
	}
	
	pub const fn empty() -> Self {
		Self { ret_pc: 0, arg_count: 0, arg0_i: 0, registers: [Register::Int(0); N] }
	}
}
