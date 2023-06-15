use super::{program::Program, instructions::Instr};

macro_rules! add_n {
	($n:ident, $t:ty) => {
		#[allow(dead_code)]
		pub fn $n(&mut self, n: $t) {
			self.program.code.append(&mut n.to_le_bytes().to_vec());
		}
	};
}

pub struct Assembler {
	pub program: Program,
	pc: usize
}

impl Assembler {
	pub const fn new() -> Self {
		Self {
			program: Program::new(),
			pc: 0
		}
	}

	pub fn add_function(&mut self, program: Program) {
		self.program.functions.push(program);
	}

	add_n!(add_u8, u8);
	add_n!(add_u16, u16);
	add_n!(add_u32, u32);
	add_n!(add_u64, u64);

	add_n!(add_i8, i8);
	add_n!(add_i16, i16);
	add_n!(add_i32, i32);
	add_n!(add_i64, i64);
	
	add_n!(add_f64, f64);

	pub fn add_instr(&mut self, instr: Instr) -> usize {
		instr.compile(self);
		let old_pc = self.pc;
		self.pc += instr.size();
		old_pc
	}
}
