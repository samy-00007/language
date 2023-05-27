use std::collections::HashMap;

use crate::parser::ast::{Expr, Literal, Operator, Stmt};

pub fn walk(block: Vec<Stmt>, locals: &mut HashMap<String, usize>, args: [usize; 2]) {
	for x in block {
		let _res = eval(x, locals, args);
	}
}

fn eval(stmt: Stmt, locals: &mut HashMap<String, usize>, args: [usize; 2]) -> usize {
	match stmt {
		Stmt::Local { name, t: _, val } => {
			let r = eval_expr(*val, locals, args);
			locals.insert(name, r);
		}
		Stmt::Expr(x) => {
			eval_expr(x, locals, args);
		}
		Stmt::While { cond, block } => {
			while eval_expr(cond.clone(), locals, args) > 0 {
				walk(block.clone(), locals, args);
			}
		}
		_ => todo!()
	};
	0
}

fn eval_expr(expr: Expr, locals: &mut HashMap<String, usize>, args: [usize; 2]) -> usize {
	match expr {
		Expr::Block(x) => walk(x, locals, args),
		Expr::Lit(x) => match x {
			Literal::Int(x) => return x as usize,
			_ => todo!()
		},
		Expr::Infix { op, lhs, rhs } => match op {
			Operator::Assign => match *lhs {
				Expr::Ident(s) => {
					let r = eval_expr(*rhs, locals, args);
					locals.insert(s, r);
				}
				_ => panic!("assign to non-ident")
			},
			Operator::Gt => {
				return if eval_expr(*lhs, locals, args) > eval_expr(*rhs, locals, args) {
					1
				} else {
					0
				}
			},
			Operator::Add => {
				return eval_expr(*lhs, locals, args) + eval_expr(*rhs, locals, args)
			},
			Operator::Sub => {
				return eval_expr(*lhs, locals, args) - eval_expr(*rhs, locals, args)
			},
			x => todo!("{}", x)
		},
		Expr::Ident(x) => return get_val(x, locals, args),
		x => todo!("{}", x)
	};
	0
}

fn get_val(ident: String, locals: &mut HashMap<String, usize>, args: [usize; 2]) -> usize {
	if ident == *"arg_0" {
		args[0]
	} else if ident == *"arg_1" {
		args[1]
	} else {
		// println!("{}", ident);
		*locals.get(&ident).unwrap()
	}
}
