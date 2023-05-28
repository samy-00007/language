use std::collections::HashMap;
use crate::parser::ast::{Stmt, Literal, Expr, Operator};
use super::Opcode;

pub fn compile_block(prog: Vec<Stmt>) -> (Vec<u8>, Vec<Literal>) {
	let mut constants = Vec::new();
	let mut constants_ci = HashMap::<String, usize>::new();
	let mut ci = 0usize;
	let mut bytecode = Vec::new();
	for x in prog {
		match x {
			Stmt::Local { name, t: _, val } => {
				compile_expr(
					*val,
					&mut bytecode,
					&mut constants,
					&mut constants_ci,
					&mut ci
				);
				let i = add_named_const(
					name.clone(),
					Literal::String(name),
					&mut constants,
					&mut constants_ci,
					&mut ci
				);
				bytecode.push(Opcode::DefGlob.into());
				bytecode.push(i as u8);
			}
			Stmt::Expr(e) => {
				compile_expr(e, &mut bytecode, &mut constants, &mut constants_ci, &mut ci);
			}
			_ => todo!()
		};
	}
	bytecode.push(Opcode::Hlt.into());
	(bytecode, constants)
}

fn compile_expr(
	expr: Expr,
	bytecode: &mut Vec<u8>,
	constants: &mut Vec<Literal>,
	constants_ci: &mut HashMap<String, usize>,
	ci: &mut usize
) {
	match expr {
		Expr::Lit(l) => {
			let i = add_const(l, constants, ci);
			bytecode.push(Opcode::Const.into());
			bytecode.push(i as u8);
		}
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
				}
				Operator::Assign => {
					compile_expr(*rhs, bytecode, constants, constants_ci, ci);
					// compile_expr(*lhs, bytecode, constants, constants_ci, ci);
					match *lhs {
						Expr::Ident(s) => {
							let i = get_const_id(s, constants_ci);
							bytecode.push(Opcode::DefGlob.into());
							//bytecode.push(Opcode::C)
							bytecode.push(i as u8);
						}
						_ => panic!("lhs of assign must be ident")
					}
				}
				_ => todo!()
			}
		}
		Expr::Ident(s) => {
			let i = *constants_ci.get(&s).unwrap();
			bytecode.push(Opcode::GetGlob.into());
			bytecode.push(i as u8);
		}
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
