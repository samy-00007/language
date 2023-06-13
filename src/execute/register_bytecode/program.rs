#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
	pub code: Vec<u8>,
	pub functions: Vec<Program>
}

impl Program {
	pub fn new() -> Self {
		Self {
			code: Vec::new(),
			functions: Vec::new()
		}
	}
}

// TODO: change that
