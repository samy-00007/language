#![allow(clippy::pedantic)]
#![allow(clippy::nursery)]
#![allow(dead_code)]

use super::{assembler::Assembler, Opcode};
use crate::parser::ast::{Expr, Literal, Operator, Stmt};
use std::collections::HashMap;

struct Scopes {
	locals: Vec<HashMap<String, usize>>,
	depths: Vec<usize>,
	depth: usize
}

impl Scopes {
	pub const fn new() -> Self {
		Self {
			locals: Vec::new(),
			depths: Vec::new(),
			depth: 0
		}
	}

	pub fn new_scope(&mut self) {
		self.locals.push(HashMap::new());
		self.depths.push(0);
	}

	pub fn pop_scope(&mut self) -> usize {
		let _vars = self.locals.pop().unwrap().values();
		let n_var = self.depths.pop().unwrap();
		self.depth -= n_var;
		n_var
	}

	pub fn new_var(&mut self, n: String) -> usize {
		self.locals.last_mut().unwrap().insert(n, self.depth);
		*self.depths.last_mut().unwrap() += 1;
		self.depth += 1;
		self.depth - 1
	}

	pub fn resolve_var(&self, n: String) -> usize {
		for s in &self.locals {
			if let Some(depth) = s.get(&n) {
				return *depth
			}
		}
		unreachable!()
		// *self.locals.last().unwrap().get(&n).unwrap()
	}
}

pub fn compile(prog: Vec<Stmt>) -> (Vec<u8>, Vec<Literal>) {
	let mut constants = Vec::new();
	let mut constants_ci = HashMap::<String, usize>::new();
	let mut ci = 0usize;
	let mut assembler = Assembler::new();
	let mut scopes = Scopes::new();

	scopes.new_scope();
	compile_block(
		prog,
		&mut assembler,
		&mut constants,
		&mut constants_ci,
		&mut ci,
		&mut scopes
	);
	let n = scopes.pop_scope();
	assembler.add_opcode(Opcode::UnsetLocal);
	assembler.add_u8(n as u8);

	assembler.add_opcode(Opcode::Hlt);
	(assembler.0, constants)
}

fn compile_block(
	block: Vec<Stmt>,
	assembler: &mut Assembler,
	constants: &mut Vec<Literal>,
	constants_ci: &mut HashMap<String, usize>,
	ci: &mut usize,
	scopes: &mut Scopes
) {
	for x in block {
		match x {
			Stmt::Local { name, t: _, val } => {
				compile_expr(
					*val,
					assembler,
					constants,
					constants_ci,
					ci,
					scopes
				);
				let i = scopes.new_var(name);
				assembler.add_opcode(Opcode::SetLocal);
				assembler.add_u8(i as u8);
				// let i = add_named_const(
				// 	name.clone(),
				// 	Literal::String(name),
				// 	&mut constants,
				// 	&mut constants_ci,
				// 	&mut ci
				// );
				// assembler.add_opcode(Opcode::DefGlob);
				// assembler.add_u8(i as u8);
			}
			Stmt::Expr(e) => {
				compile_expr(
					e,
					assembler,
					constants,
					constants_ci,
					ci,
					scopes
				);
			}
			_ => todo!()
		};
	}
}

fn compile_expr(
	expr: Expr,
	assembler: &mut Assembler,
	constants: &mut Vec<Literal>,
	constants_ci: &mut HashMap<String, usize>,
	ci: &mut usize,
	scopes: &mut Scopes
) {
	match expr {
		Expr::Lit(l) => {
			let i = add_const(l, constants, ci);
			assembler.add_opcode(Opcode::Const);
			assembler.add_u8(i as u8);
		}
		Expr::Infix { op, lhs, rhs } => {
			match op {
				Operator::Add | Operator::Sub | Operator::Mul | Operator::Div => {
					compile_expr(*lhs, assembler, constants, constants_ci, ci, scopes);
					compile_expr(*rhs, assembler, constants, constants_ci, ci, scopes);
					assembler.add_opcode(match op {
						Operator::Add => Opcode::Add,
						Operator::Sub => Opcode::Sub,
						Operator::Mul => Opcode::Mul,
						Operator::Div => Opcode::Div,
						_ => unreachable!()
					});
				}
				Operator::Assign => {
					compile_expr(*rhs, assembler, constants, constants_ci, ci, scopes);
					// compile_expr(*lhs, bytecode, constants, constants_ci, ci);
					match *lhs {
						Expr::Ident(s) => {
							let i = scopes.resolve_var(s);
							assembler.add_opcode(Opcode::SetLocal);
							assembler.add_u8(i as u8);
							// let i = get_const_id(s, constants_ci);
							// bytecode.push(Opcode::DefGlob.into());
							//bytecode.push(Opcode::C)
							// bytecode.push(i as u8);
						}
						_ => panic!("lhs of assign must be ident")
					}
				}
				_ => todo!()
			}
		}
		Expr::Ident(s) => {
			// let i = *constants_ci.get(&s).unwrap();
			// assembler.add_opcode(Opcode::GetGlob);
			// assembler.add_u8(i as u8);
			let i = scopes.resolve_var(s);
			assembler.add_opcode(Opcode::GetLocal);
			assembler.add_u8(i as u8);
		}
		Expr::FnNamedCall { name, args } => {
			if name == *"print" {
				let arg = args[0].clone();
				compile_expr(arg, assembler, constants, constants_ci, ci, scopes);
				assembler.add_opcode(Opcode::Print);
			} else {
				panic!()
			}
		}
		Expr::Block(block) => {
			scopes.new_scope();
			compile_block(block, assembler, constants, constants_ci, ci, scopes);
			let n = scopes.pop_scope();
			assembler.add_opcode(Opcode::UnsetLocal);
			assembler.add_u8(n as u8);
		}
		_ => todo!()
	}
}

fn add_named_const(
	n: String,
	l: Literal,
	constants: &mut Vec<Literal>,
	constants_ci: &mut HashMap<String, usize>,
	ci: &mut usize
) -> usize {
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

fn get_const_id(n: String, constants_ci: &mut HashMap<String, usize>) -> usize {
	*constants_ci.get(&n).unwrap()
}
