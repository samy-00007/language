#![allow(clippy::cast_precision_loss)]
#![allow(dead_code)]
// use std::collections::HashMap;
use rustc_hash::FxHashMap as HashMap;

use crate::parser::ast::{Expr, Literal, Operator, Stmt};

pub fn walk(block: Vec<Stmt>, locals: &mut HashMap<String, Literal>, args: [usize; 2]) {
	for x in block {
		let _res = eval(x, locals, args);
	}
}

fn eval(stmt: Stmt, locals: &mut HashMap<String, Literal>, args: [usize; 2]) -> usize {
	match stmt {
		Stmt::Local { name, t: _, val } => {
			let r = eval_expr(*val, locals, args);
			locals.insert(name, r);
		}
		Stmt::Expr(x) => {
			eval_expr(x, locals, args);
		}
		Stmt::While { cond, block } => {
			while eval_expr(cond.clone(), locals, args) == Literal::Bool(true) {
				walk(block.clone(), locals, args);
			}
		}
		_ => todo!()
	};
	0
}

fn eval_expr(expr: Expr, locals: &mut HashMap<String, Literal>, args: [usize; 2]) -> Literal {
	match expr {
		Expr::Block(x) => walk(x, locals, args),
		Expr::Lit(x) => match x {
			Literal::Int(x) => return Literal::Int(x),
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
					Literal::Bool(true)
				} else {
					Literal::Bool(false)
				}
			}
			Operator::Add => return eval_expr(*lhs, locals, args) + eval_expr(*rhs, locals, args),
			Operator::Sub => {
				let a = eval_expr(*lhs, locals, args);
				let b = eval_expr(*rhs, locals, args);
				//println!("sub {} {}", a, b);
				return a - b;
			}
			Operator::Lt => {
				return if eval_expr(*lhs, locals, args) < eval_expr(*rhs, locals, args) {
					Literal::Bool(true)
				} else {
					Literal::Bool(false)
				}
			}
			x => todo!("{:?}", x)
		},
		Expr::Ident(x) => return get_val(x.as_str(), locals, args),
		Expr::FnNamedCall { name, args: argus } => {
			if name == "clock" {
				let now = std::time::SystemTime::now();
				let since_the_epoch = now
					.duration_since(std::time::UNIX_EPOCH)
					.expect("Time went backwards");
				let ms = since_the_epoch.as_millis() as i128;
				return Literal::Int(ms);
			} else if name == "print" {
				let r = eval_expr(argus[0].clone(), locals, args);
				println!("{r}");
			}
		}
		x => todo!("{:?}", x)
	};
	Literal::Int(0)
}

fn get_val(ident: &str, locals: &mut HashMap<String, Literal>, args: [usize; 2]) -> Literal {
	if ident == "arg_0" {
		Literal::Int(args[0] as i128)
	} else if ident == "arg_1" {
		Literal::Int(args[1] as i128)
	} else {
		// println!("{}", ident);
		locals.get(&ident.to_string()).unwrap().clone()
	}
}
