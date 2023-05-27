use crate::parser::ast::{Expr, Stmt};

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Instr {

}


struct Program {
	stack: Vec<f64>,
	ip: usize,
	program: Vec<u8>
}

impl Program {
	pub fn new(program: Vec<u8>) -> Self {
		Self {
			stack: Vec::new(),
			ip: 0,
			program
		}
	}
}