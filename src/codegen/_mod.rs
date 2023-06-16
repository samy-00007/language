use crate::{parser::ast::{Stmt, Ty, Expr, Literal, Operator, Prefix}, vm::assembler::Assembler};
use std::collections::{HashMap, HashSet};
use super::vm::program::Program;
use std::intrinsics::likely;
use super::Instr;

use core::ops::{Add, Sub, Mul, Div};



macro_rules! stack_op {
	($trait:ident, $name:ident, $op:tt, $all_floats:literal) => {
		impl $trait for Literal {
			type Output = Self;

			fn $name(self, rhs: Self) -> Self::Output {
				match self {
					Self::Int(x) => match rhs {
						Self::Int(y) => if $all_floats { Self::Float(x as f64 $op y as f64) } else { Self::Int(x $op y) },
						Self::Float(y) => Self::Float(x as f64 $op y),
						_ => unreachable!()
					},
					Self::Float(x) => match rhs {
						Self::Float(y) => Self::Float(x $op y),
						Self::Int(y) => Self::Float(x $op y as f64),
						_ => unreachable!()
					},
					_ => unreachable!()
				}
			}
		}
	};
}

stack_op!(Add, add, +, false);
stack_op!(Sub, sub, -, false);
stack_op!(Mul, mul, -, false);
stack_op!(Div, div, /, true);




//type, reg
struct Var(String, u8);


struct Env {
	scopes: Vec<HashMap<String, Var>>,
	regs: Vec<u8>
}

impl Env {
	fn new() -> Self {
		Self { scopes: Vec::new(), regs: Vec::new() }
	}

	fn push_scope(&mut self) {
		self.scopes.push(HashMap::new());
		self.regs.push(0);
	}

	fn pop_scope(&mut self) {
		self.scopes.pop().unwrap();
		self.regs.pop().unwrap();
	}

	fn exists_var(&self, name: &String) -> bool {
		for scope in &self.scopes {
			if scope.contains_key(name) {
				return true
			}
		}
		false
	}

	fn define_var(&mut self, name: String, ty: String) -> u8 {
		let last_reg = self.regs.last_mut().unwrap();
		*last_reg += 1;
		self.scopes.last_mut().unwrap().insert(name, Var(ty, *last_reg - 1));
		*last_reg - 1
	}

}

struct Compiler {
	assembler: Assembler,
	env: Env
}
impl Compiler {
	pub fn new() -> Self {
		Self { assembler: Assembler::new(), env: Env::new() }
	}

	fn compile_block(&mut self, block: Vec<Stmt>) -> Program {
		let mut assembler = Assembler::new();
		for stmt in block {
			match stmt {
				Stmt::Error => unreachable!(),
				Stmt::Local { name, t, val } => {
					assert!(!self.env.exists_var(&name), "a variable already has this name in this scope");

					let Ty::Ident(ty) = t.unwrap() else {unreachable!()};
					let reg = self.env.define_var(name, ty.clone());

					// self.compile_expr(&mut assembler, *val);

					match *val {
						Expr::Lit(lit) => {
							match lit {
								Literal::Bool(x) => {
									assert!(ty == "bool");
									if x {
										assembler.add_instr(Instr::LoadTrue(reg));
									} else {
										assembler.add_instr(Instr::LoadFalse(reg));
									}
								}
								Literal::Int(x) => {
									assert!(ty == "number");
									assembler.add_instr(Instr::Load(reg, x));
								}
								Literal::Float(x) => {
									assert!(ty == "number");
									assembler.add_instr(Instr::LoadFloat(reg, x));
								}
								Literal::String(_x) => {
									assert!(ty == "string");
									unimplemented!();
								}
							}
						},
						_ => unimplemented!()
					}



				}
				_ => unimplemented!()
			}
		}
		assembler.program
	}

	fn compile_expr(&mut self, assembler: &mut Assembler, expr: Expr) {}

}



/*

pub enum Expr {
	Ident(String),
	Lit(Literal),
	Prefix(Prefix, E),
	Infix { op: Operator, lhs: E, rhs: E },
	Block(Block),
	FnCall { expr: E, args: Vec<Expr> },
	FnNamedCall { name: String, args: Vec<Expr> },
	Error
}

 */
fn is_constant(expr: Expr) -> bool {
	match expr {
		Expr::Ident(_) => false, // TODO: check if the val of the ident (fn or variable) is constant
		Expr::Lit(_) => true,
		Expr::Infix { op: _, lhs, rhs } => is_constant(*lhs) && is_constant(*rhs),
		Expr::Prefix(_, e) => is_constant(*e),
		Expr::FnNamedCall { name: _, args: _ } => unimplemented!(),
		Expr::FnCall { expr: _, args: _ } => unimplemented!(),
		Expr::Block(_) => unimplemented!(),
		Expr::Error => unreachable!()
	}
}

fn compute_constant_expr(expr: Expr) -> Literal {
	match expr {
		Expr::Lit(x) => x,
		Expr::Prefix(prefix, expr) => compute_prefix(prefix, *expr),
		Expr::Infix { op, lhs, rhs } => compute_infix(op, *lhs, *rhs),
		Expr::Block(_) | Expr::FnCall { expr: _, args: _ } | Expr::FnNamedCall { name: _, args: _ } => todo!(),
		Expr::Error | Expr::Ident(_) => unreachable!()
	}
}

fn compute_infix(op: Operator, lhs: Expr, rhs: Expr) -> Literal {
	let lhs = compute_constant_expr(lhs);
	let rhs = compute_constant_expr(rhs);
	match op {
		Operator::Add => lhs + rhs,
		Operator::Sub => lhs - rhs,
		Operator::Mul => lhs * rhs,
		Operator::Div => lhs / rhs,
		Operator::And => {
			assert!(matches!(lhs, Literal::Bool(_)));
			assert!(matches!(rhs, Literal::Bool(_)));
			todo!() //lhs & rhs
		},
		Operator::BitAnd => {
			assert!(matches!(lhs, Literal::Int(_) | Literal::Float(_)));
			assert!(matches!(rhs, Literal::Int(_) | Literal::Float(_)));
			todo!() //lhs & rhs	
		},
		Operator::BitOr => todo!(),//lhs | rhs,
		Operator::BitXor => todo!(),//lhs ^ rhs,
		Operator::Exponent => todo!(),//lhs.pow(rhs),
		Operator::Gt => Literal::Bool(lhs > rhs),
		Operator::Gte => Literal::Bool(lhs >= rhs),
		Operator::Lt => Literal::Bool(lhs < rhs),
		Operator::Lte => Literal::Bool(lhs <= rhs),
		Operator::Eq => Literal::Bool(lhs == rhs),
		Operator::Neq => Literal::Bool(lhs == rhs),
		Operator::LShift => todo!(),//lhs << rhs,
		Operator::RShift => todo!(),//lhs >> rhs,
		Operator::Rem => todo!(),//lhs % rhs,
		_ => unreachable!()
	}
}


fn compute_prefix(prefix: Prefix, expr: Expr) -> Literal {
	let val = compute_constant_expr(expr);
	match prefix {
		Prefix::BitNot => {
			if let Literal::Int(x) = val {
				Literal::Int(!x) // ! if bitwise not for numbers in rust
			} else {
				panic!("Prefix 'bitnot' can only be applied to integers")
			}
		},
		Prefix::Not => {
			if let Literal::Bool(x) = val {
				Literal::Bool(!x)
			} else {
				panic!("Prefix 'not' can only be applied to bools")
			}
		},
		Prefix::Plus => {
			assert!(matches!(val, Literal::Float(_) | Literal::Int(_)));
			val
		},
		Prefix::Minus => {
			if let Literal::Int(x) = val {
				Literal::Int(-x)
			} else if let Literal::Float(x) = val {
				Literal::Float(-x)
			} else {
				panic!("Prefix 'minus' can only be applied to numbers")
			}
		},
		Prefix::Err => unreachable!()
	}
}
