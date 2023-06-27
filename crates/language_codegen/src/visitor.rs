use language_engine::vm::{program::Program, opcodes::{Opcode, Reg, Address, Lit}};

pub fn get_bytecode(program: &Program) -> String {
	let code = &program.code;
	let functions = &program.functions;

	let mut v = Visitor::new(code);
	
	let mut bytecode = "-- root --\n\n".to_string();
	bytecode += &v.vec_to_bytecode();
	bytecode += "\n\n-- root END --\n\n";
	
	for (i, f) in functions.iter().enumerate() {
		let mut v = Visitor::new(&f.code);

		bytecode += &format!("\n\n-- function {} --\n\n", i);
		bytecode += &v.vec_to_bytecode();
		bytecode += &format!("\n\n-- function {} END --\n\n", i);
	}

	bytecode
}


pub struct Visitor<'a> {
	i: usize,
	bytecode: &'a Vec<u8>
}

macro_rules! gen_read {
	($name:ident, $t:tt) => {
		fn $name(&mut self) -> $t {
			let size = std::mem::size_of::<$t>();
			let bytes = &self.bytecode[self.i..(self.i + size)];
			self.i += size;
			$t::from_le_bytes(bytes.try_into().unwrap())
		} 
	};
}

impl<'a> Visitor<'a> {
	pub fn new(bytecode: &'a Vec<u8>) -> Self {
		Self { i: 0, bytecode }
	}

	pub fn vec_to_bytecode(&mut self) -> String {
		let mut asm: Vec<String> = Vec::new();
		loop {
			let i = self.i;
			if self.i == self.bytecode.len() {
				break;
			}
			let op: Opcode = self.read_opcode();

			let str = match op {
				Opcode::Halt => "HALT".to_string(),
				Opcode::Nop => "NOP".to_string(),
				Opcode::Load => format!("LOAD {} {}", self.read_reg(), self.read_lit()),
				Opcode::Move => format!("MOVE {} {}", self.read_reg(), self.read_reg()),
				Opcode::Jmp => format!("JMP {}", self.read_address()),
				Opcode::JmpIfTrue => format!("JmpIfTrue {} {}", self.read_reg(), self.read_address()),
				Opcode::JmpIfFalse => format!("JmpIfFalse {} {}", self.read_reg(), self.read_address()),
				Opcode::Add => format!("ADD {} {} {}", self.read_reg(), self.read_reg(), self.read_reg()),
				Opcode::Sub => format!("SUB {} {} {}", self.read_reg(), self.read_reg(), self.read_reg()),
				Opcode::Mul => format!("MUL {} {} {}", self.read_reg(), self.read_reg(), self.read_reg()),
				Opcode::Div => format!("DIV {} {} {}", self.read_reg(), self.read_reg(), self.read_reg()),
				Opcode::Lt => format!("LT {} {} {}", self.read_reg(), self.read_reg(), self.read_reg()),
				Opcode::Addl => format!("ADDL {} {} {}", self.read_reg(), self.read_reg(), self.read_lit()),
				Opcode::Subl => format!("SUBL {} {} {}", self.read_reg(), self.read_reg(), self.read_lit()),
				Opcode::Mull => format!("MULL {} {} {}", self.read_reg(), self.read_reg(), self.read_lit()),
				Opcode::Divl => format!("DIVL {} {} {}", self.read_reg(), self.read_reg(), self.read_lit()),
				Opcode::Ltl => format!("LTL {} {} {}", self.read_reg(), self.read_reg(), self.read_lit()),
				Opcode::Clock => format!("CLOCK {}", self.read_reg()),
				Opcode::Call => format!("CALL {} {} {}", self.read_reg(), self.read_u8(), self.read_u8()),
				Opcode::Ret => format!("RET {} {}", self.read_reg(), self.read_u8()),
				Opcode::LoadF => format!("LOADF {} {}", self.read_reg(), self.read_u16()),
				Opcode::LoadTrue => format!("LOADTRUE {}", self.read_reg()),
				Opcode::LoadFalse => format!("LOADFALSE {}", self.read_reg()),
				Opcode::LoadFloat => format!("LOADFLOAT {} {}", self.read_reg(), self.read_f64()),
				Opcode::Print => format!("PRINT {}", self.read_reg())
			};
			asm.push(format!("{} - {}", i, str));
		}
		asm.join("\n")
	}


	fn read_u8(&mut self) -> u8 {
		let val = self.bytecode[self.i];
		self.i += 1;
		val
	}
	gen_read!(read_u16, u16);
	gen_read!(read_i64, i64);
	gen_read!(read_f64, f64);

	fn read_opcode(&mut self) -> Opcode {
		self.read_u8().into()
	}

	fn read_reg(&mut self) -> Reg {
		self.read_u8()
	}

	fn read_address(&mut self) -> Address {
		self.read_u16()
	}

	fn read_lit(&mut self) -> Lit {
		self.read_i64()
	}
}