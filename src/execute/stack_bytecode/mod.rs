use std::collections::HashMap;

use crate::parser::ast::{Expr, Stmt, Literal, Operator};

// https://blog.subnetzero.io/post/building-language-vm-part-02/
// https://craftinginterpreters.com/a-virtual-machine.html


macro_rules! op {
	($self:ident, $op:tt) => {
		{
			let b = $self.stack.pop().unwrap();
			let a = $self.stack.pop().unwrap();
			$self.stack.push((a $op b))
		}
	};
}


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
			Opcode::Igl => 255
		}
	}
}


#[derive(Debug)]
pub struct Program {
	stack: Vec<Literal>,
	pc: usize,
	program: Vec<u8>,
	constants: Vec<Literal>,
	ci: usize,
	globals: HashMap<String, Literal>
}

impl Program {
	pub fn new(program: Vec<u8>, constants: Vec<Literal>) -> Self {
		Self {
			stack: Vec::new(),
			pc: 0,
			program,
			constants,
			ci: 0,
			globals: HashMap::new()
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
				break
			}
			match self.decode_opcode() {
				Opcode::Hlt => {
					println!("HLT");
					return
				},
				Opcode::Const => {
					let constant = self.next_constant();
					self.stack.push(constant);
				},
				Opcode::Neg => {
					let old = self.stack.pop().unwrap();
					// ideally unreachable!() with type check at compile time instead of runtime
					self.stack.push(-old);
				},
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
				},
				Opcode::GetGlob => {
					let constant = self.next_constant();
					match constant {
						Literal::String(s) => self.stack.push(self.globals.get(&s).unwrap().clone()),
						_ => panic!("var name must be a string (2)")
					}
				},
				_ => {
					eprintln!("Unknown opcode found. Terminating.");
					return
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



pub fn compile_block(prog: Vec<Stmt>) -> (Vec<u8>, Vec<Literal>) {
	let mut constants = Vec::new();
	let mut constants_ci = HashMap::<String, usize>::new();
	let mut ci = 0usize;
	let mut bytecode = Vec::new();
	for x in prog {
		match x {
			Stmt::Local { name, t: _, val } => {
				compile_expr(*val, &mut bytecode, &mut constants, &mut constants_ci, &mut ci);
				let i = add_named_const(name.clone(), Literal::String(name), &mut constants, &mut constants_ci, &mut ci);
				bytecode.push(Opcode::DefGlob.into());
				bytecode.push(i as u8);
			},
			Stmt::Expr(e) => compile_expr(e, &mut bytecode, &mut constants, &mut constants_ci, &mut ci),
			_ => todo!()
		};
	}
	bytecode.push(Opcode::Hlt.into());
	(bytecode, constants)
}

fn compile_expr(expr: Expr, bytecode: &mut Vec<u8>, constants: &mut Vec<Literal>, constants_ci: &mut HashMap<String, usize>, ci: &mut usize) {
	match expr {
		Expr::Lit(l) => {
			let i = add_const(l, constants, ci);
			bytecode.push(Opcode::Const.into());
			bytecode.push(i as u8);
		},
		Expr::Infix { op, lhs, rhs } => {
			
			match op {
				Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
					compile_expr(*lhs, bytecode, constants, constants_ci, ci);
					compile_expr(*rhs, bytecode, constants, constants_ci, ci);
					bytecode.push(match op {
						Operator::Add => Opcode::Add.into(),
						Operator::Sub => Opcode::Sub.into(),
						Operator::Mul => Opcode::Mul.into(),
						Operator::Div => Opcode::Div.into(),
						_ => unreachable!()
					});
				},
				Operator::Assign => {
					compile_expr(*rhs, bytecode, constants, constants_ci, ci);
					// compile_expr(*lhs, bytecode, constants, constants_ci, ci);
					match *lhs {
						Expr::Ident(s) => {
							let i = get_const_id(s, constants_ci);
							bytecode.push(Opcode::DefGlob.into());
							bytecode.push(i as u8);
						},
						_ => panic!("lhs of assign must be ident")
					}
				},
				_ => todo!()
			}
			
			
		},
		Expr::Ident(s) => {
			let i = *constants_ci.get(&s).unwrap();
			bytecode.push(Opcode::GetGlob.into());
			bytecode.push(i as u8);
		},
		Expr::FnNamedCall { name, args } => {
			if name == *"print" {
				let arg = args[0].clone();
				compile_expr(arg, bytecode, constants, constants_ci, ci);
				bytecode.push(Opcode::Print.into());
			} else {
				panic!()
			}
		}
		_ => todo!()
	}
}

fn add_named_const(n: String, l: Literal, constants: &mut Vec<Literal>, constants_ci: &mut HashMap<String, usize>, ci: &mut usize) -> usize {
	constants.push(l);
	constants_ci.insert(n, *ci);
	*ci += 1;
	*ci - 1
}

fn add_const(l: Literal, constants: &mut Vec<Literal>, ci: &mut usize) -> usize {
	constants.push(l);
	*ci += 1;
	*ci - 1
}

fn get_const_id(n: String, constants_ci: &mut HashMap<String, usize> ) -> usize {
	*constants_ci.get(&n).unwrap()
}