pub mod assembler;
pub mod compiler;
pub mod vm;

// https://blog.subnetzero.io/post/building-language-vm-part-02/
// https://craftinginterpreters.com/a-virtual-machine.html

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Opcode {
	Hlt,
	Const,
	Neg,
	Add, // TODO: math ops as a single instr with the operator as a 2nd byte, index for an array of fn
	Sub,
	Mul,
	Div,
	Print,
	DefGlob,
	GetGlob,
	SetLocal,
	UnsetLocal,
	GetLocal,
	Time,
	Lt,
	Jmp,
	Jmpn,
	Igl // illegal
}

impl From<u8> for Opcode {
	fn from(value: u8) -> Self {
		match value {
			0 => Self::Hlt,
			1 => Self::Const,
			2 => Self::Neg,
			3 => Self::Add,
			4 => Self::Sub,
			5 => Self::Mul,
			6 => Self::Div,
			7 => Self::Print,
			8 => Self::DefGlob,
			9 => Self::GetGlob,
			10 => Self::SetLocal,
			11 => Self::UnsetLocal,
			12 => Self::GetLocal,
			13 => Self::Time,
			14 => Self::Lt,
			15 => Self::Jmp,
			16 => Self::Jmpn,
			_ => Self::Igl
		}
	}
}

impl From<Opcode> for u8 {
	fn from(value: Opcode) -> Self {
		match value {
			Opcode::Hlt => 0,
			Opcode::Const => 1,
			Opcode::Neg => 2,
			Opcode::Add => 3,
			Opcode::Sub => 4,
			Opcode::Mul => 5,
			Opcode::Div => 6,
			Opcode::Print => 7,
			Opcode::DefGlob => 8,
			Opcode::GetGlob => 9,
			Opcode::SetLocal => 10,
			Opcode::UnsetLocal => 11,
			Opcode::GetLocal => 12,
			Opcode::Time => 13,
			Opcode::Lt => 14,
			Opcode::Jmp => 15,
			Opcode::Jmpn => 16,
			Opcode::Igl => 255
		}
	}
}
