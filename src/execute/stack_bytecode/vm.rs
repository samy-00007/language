#![allow(dead_code)]

use std::collections::HashMap;
use crate::parser::ast::Literal;
use super::Opcode;

macro_rules! op {
	($self:ident, $op:tt) => {
		{
			let b = $self.stack.pop().unwrap();
			let a = $self.stack.pop().unwrap();
			$self.stack.push((a $op b))
		}
	};
}

#[derive(Debug)]
pub struct Vm {
	stack: Vec<Literal>,
	pc: usize,
	program: Vec<u8>,
	constants: Vec<Literal>,
	ci: usize,
	globals: HashMap<String, Literal>,
	locals: Vec<Literal>
}

impl Vm {
	pub fn new(program: Vec<u8>, constants: Vec<Literal>) -> Self {
		Self {
			stack: Vec::new(),
			pc: 0,
			program,
			constants,
			ci: 0,
			globals: HashMap::new(),
			locals: Vec::new()
		}
	}

	fn push_constant(&mut self, c: Literal) -> usize {
		self.constants.push(c);
		self.ci += 1;
		self.ci
	}

	fn get_constant(&self, i: usize) -> Literal {
		self.constants[i].clone()
	}

	pub fn run(&mut self) {
		loop {
			if self.pc >= self.program.len() {
				break;
			}
			match self.decode_opcode() {
				Opcode::Hlt => {
					println!("HLT");
					return;
				}
				Opcode::Const => {
					let constant = self.next_constant();
					self.stack.push(constant);
				}
				Opcode::Neg => {
					let old = self.stack.pop().unwrap();
					// ideally unreachable!() with type check at compile time instead of runtime
					self.stack.push(-old);
				}
				Opcode::Add => op!(self, +),
				Opcode::Sub => op!(self, -),
				Opcode::Mul => op!(self, *),
				Opcode::Div => op!(self, /),
				Opcode::Print => println!("{}", self.stack.pop().unwrap()),
				Opcode::DefGlob => {
					let constant = self.next_constant();
					let val = self.stack.pop().unwrap();
					match constant {
						Literal::String(s) => self.globals.insert(s, val),
						_ => panic!("var name must be a string")
					};
				}
				Opcode::GetGlob => {
					let constant = self.next_constant();
					match constant {
						Literal::String(s) => {
							self.stack.push(self.globals.get(&s).unwrap().clone());
						}
						_ => panic!("var name must be a string (2)")
					}
				},
				Opcode::SetLocal => {
					// println!("{}", self.pc);
					let i = self.next_u8();
					let val = self.stack.pop().unwrap();
					if i as usize == self.locals.len() {
						self.locals.push(val);
					} else {
						// println!("{}", self.pc);
						self.locals[i as usize] = val;
					}
				},
				Opcode::GetLocal => {
					let i = self.next_u8();
					let val = self.locals[i as usize].clone();
					self.stack.push(val);
				},
				Opcode::UnsetLocal => {
					let n = self.next_u8();
					let new_len = self.locals.len().saturating_sub(n as usize);
					self.locals.truncate(new_len);
				}
				Opcode::Igl => {
					eprintln!("Unknown opcode found. Terminating.");
					return;
				}
			}
		}
	}

	fn next_constant(&mut self) -> Literal {
		let i = self.next_u8() as usize;
		self.get_constant(i)
	}

	fn next_u8(&mut self) -> u8 {
		self.next()
	}

	fn next(&mut self) -> u8 {
		let n = self.program[self.pc];
		self.pc += 1;
		n
	}

	fn next_u64(&mut self) -> usize {
		let bytes = &self.program[self.pc..(self.pc + 8)];
		self.pc += 8;
		usize::from_be_bytes(bytes.try_into().unwrap())
	}

	fn decode_opcode(&mut self) -> Opcode {
		Opcode::from(self.next())
	}
}
