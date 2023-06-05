use super::Instr;

macro_rules! add_n {
	($n:ident, $t:ty) => {
		pub fn $n(&mut self, n: $t) {
			self.program.append(&mut n.to_le_bytes().to_vec());
		}
	};
}

pub struct Assembler {
	pub program: Vec<u8>,
	pc: usize
}

impl Assembler {
	pub const fn new() -> Self {
		Self {
			program: Vec::new(),
			pc: 0
		}
	}

	add_n!(add_u8, u8);
	add_n!(add_u16, u16);
	add_n!(add_u32, u32);
	add_n!(add_u64, u64);

	pub fn add_instr(&mut self, instr: Instr) {
		instr.compile(self);
	}
}
