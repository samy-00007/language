use language_engine::vm::{instructions::Instr, program::Program};

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

	unsafe fn compile_instr(instr: Instr) -> &'static mut Vec<u8> {
		static mut BUFFER: Vec<u8> = Vec::new();

		#[cfg(debug_assertions)]
		println!("{}", instr.to_string());

		instr.compile(&mut BUFFER);

		&mut BUFFER
	}

	pub fn add_instr(&mut self, instr: Instr) -> usize {
		let old_len = self.program.code.len();

		let instr = unsafe { Self::compile_instr(instr) };

		self.program.code.append(instr);

		old_len
	}

	/// This function compile the instruction into bytes and replace
	/// `i..i+n` with the new instruction, with `n` the number of bytes of
	/// the compiled instruction. The caller must ensure that the replaced instruction
	/// is exactly the same size as the new one.
	pub fn set_instr(&mut self, i: usize, instr: Instr) {
		let instr = unsafe { Self::compile_instr(instr) }.clone();

		let range = i..(i + instr.len());

		self.program.code.splice(range, instr);
	}
}
