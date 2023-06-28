use super::stack::StackValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
	pub code: Vec<u8>,
	pub returned: bool,
	pub functions: Vec<Program>,
	pub constants: Vec<StackValue>
}

impl Program {
	pub const fn new() -> Self {
		Self {
			code: Vec::new(),
			returned: false,
			functions: Vec::new(),
			constants: Vec::new()
		}
	}
}

// TODO: change that
