use language_engine::vm::{opcodes::Opcode, program::Program};

macro_rules! emit_num {
	($name:ident, $name_:ident, $t:tt) => {
		pub fn $name(&mut self, n: $t) -> usize {
			let old_len = self.program.code.len();
			let bytes = n.to_le_bytes();
			self.program.code.extend(bytes);
			old_len
		}

		pub fn $name_(&mut self, i: usize, n: $t) {
			let range = i..(i + std::mem::size_of::<$t>());
			let bytes = n.to_le_bytes();
			self.program.code.splice(range, bytes);
		}
	};
}

#[derive(Debug)]
pub struct Assembler {
	pub program: Program
}

impl Assembler {
	pub const fn new() -> Self {
		Self {
			program: Program::new()
		}
	}

	pub fn add_function(&mut self, program: Program) -> usize {
		let i = self.program.functions.len();
		self.program.functions.push(program);
		i
	}

	pub fn emit_opcode(&mut self, op: Opcode) -> usize {
		let old_len = self.program.code.len();

		self.emit_u8(op as u8);

		old_len
	}

	emit_num!(emit_u8, set_u8, u8);
	emit_num!(emit_u16, set_u16, u16);
	emit_num!(emit_i64, set_i64, i64);
	emit_num!(emit_f64, set_f64, f64);

}
