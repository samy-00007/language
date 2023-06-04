#![allow(dead_code)]

use super::Opcode;
use crate::parser::ast::Literal;
use std::collections::HashMap;

macro_rules! op {
	($self:ident, $op:tt) => {
		{
			let b = $self.pop_stack();
			let a = $self.pop_stack();
			$self.push_stack((a $op b))
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

	fn push_stack(&mut self, x: Literal) {
		self.stack.push(x);
	}

	fn pop_stack(&mut self) -> Literal {
		self.stack.pop().unwrap()
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
					self.push_stack(constant);
				}
				Opcode::Neg => {
					let old = self.pop_stack();
					// ideally unreachable!() with type check at compile time instead of runtime
					self.push_stack(-old);
				}
				Opcode::Add => op!(self, +),
				Opcode::Sub => op!(self, -),
				Opcode::Mul => op!(self, *),
				Opcode::Div => op!(self, /),
				Opcode::Lt => {
					let b = self.pop_stack();
					let a = self.pop_stack();
					self.push_stack(Literal::Bool(a < b));
				}
				Opcode::Print => println!("{}", self.pop_stack()),
				Opcode::DefGlob => {
					let constant = self.next_constant();
					let val = self.pop_stack();
					match constant {
						Literal::String(s) => self.globals.insert(s, val),
						_ => panic!("var name must be a string")
					};
				}
				Opcode::GetGlob => {
					let constant = self.next_constant();
					match constant {
						Literal::String(s) => {
							self.push_stack(self.globals.get(&s).unwrap().clone());
						}
						_ => panic!("var name must be a string (2)")
					}
				}
				Opcode::SetLocal => {
					// println!("{}", self.pc);
					let i = self.next_u8();
					let val = self.pop_stack();
					if i as usize == self.locals.len() {
						self.locals.push(val);
					} else {
						// println!("{}", self.pc);
						self.locals[i as usize] = val;
					}
				}
				Opcode::GetLocal => {
					let i = self.next_u8();
					let val = self.locals[i as usize].clone();
					self.push_stack(val);
				}
				Opcode::UnsetLocal => {
					let n = self.next_u8();
					let new_len = self.locals.len().saturating_sub(n as usize);
					self.locals.truncate(new_len);
				}
				Opcode::Time => {
					let now = std::time::SystemTime::now();
					let since_the_epoch = now
						.duration_since(std::time::UNIX_EPOCH)
						.expect("Time went backwards");
					let ms = since_the_epoch.as_millis() as i128;
					self.push_stack(Literal::Int(ms));
				}
				Opcode::Jmpn => {
					let cond = self.pop_stack();
					let add = self.next_u8();
					if cond == Literal::Bool(false) {
						self.pc = add as usize;
					}
				}
				Opcode::Jmp => {
					let add = self.next_u8();
					self.pc = add as usize;
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
