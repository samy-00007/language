#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
	pub code: Vec<u8>,
	pub returned: bool,
	pub functions: Vec<Program>
}

impl Program {
	pub const fn new() -> Self {
		Self {
			code: Vec::new(),
			returned: false,
			functions: Vec::new()
		}
	}
}

// TODO: change that
