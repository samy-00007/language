use crate::utils::stack::Stack;

use super::vm::Register;

const CALL_STACK_SIZE: usize = 256;
const REGISTER_COUNT: usize = 256;

pub struct CallStack {
	stack: [CallFrame; CALL_STACK_SIZE],
	pc: usize
}

impl Stack for CallStack {
	type Value = CallFrame;

	fn push(&mut self, val: Self::Value) {
		assert!(self.pc < CALL_STACK_SIZE - 1);
		self.stack[self.pc] = val;
		self.pc += 1;
	}

	fn pop(&mut self) -> Self::Value {
		assert!(self.pc > 0);
		self.pc -= 1;
		self.stack[self.pc]
	}

	fn get(&mut self, i: usize) -> Self::Value {
		assert!(i < CALL_STACK_SIZE);
		*self.stack.get(i).unwrap()
	}

	fn set(&mut self, i: usize, val: Self::Value) {
		assert!(i < CALL_STACK_SIZE);
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

impl CallStack {
	pub const fn new() -> Self {
		Self { stack: [CallFrame::empty(); CALL_STACK_SIZE], pc: 1 }
	}
}



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CallFrame {
	pub ret_pc: usize,
	pub arg_count: usize,
	pub arg0_i: usize,
	pub registers: [Register; REGISTER_COUNT]
}

impl CallFrame {
	pub const fn new(ret_pc: usize, arg_count: usize, arg0_i: usize) -> Self {
		Self { ret_pc, arg_count, arg0_i, registers: [Register::Int(0); REGISTER_COUNT] }
	}
	
	pub const fn empty() -> Self {
		Self { ret_pc: 0, arg_count: 0, arg0_i: 0, registers: [Register::Int(0); REGISTER_COUNT] }
	}
}
