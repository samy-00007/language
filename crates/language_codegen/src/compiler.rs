use super::{assembler::Assembler, env::Env};
use language_ast::{Argument, Expr, Item, Literal, Operator, Prefix, Stmt, Ty};
use language_engine::vm::{
	opcodes::{Address, Opcode, Reg},
	program::Program
};

#[derive(Debug)]
pub struct Compiler {
	pub assembler: Assembler,
	pub env: Env
}

impl Compiler {
	fn compile_expr(&mut self, reg: u8, expr: Expr) -> Reg {
		match expr {
			Expr::Lit(x) => {
				self.load_lit(reg, x);
				reg
			}
			Expr::Ident(x) => self.env.get_var_reg(&x),
			Expr::Infix { op, lhs, rhs } => {
				if op == Operator::Assign {
					assert!(matches!(*lhs, Expr::Ident(_)));
					let Expr::Ident(name) = *lhs else {unreachable!()};
					let reg = self.env.get_var_reg(&name);

					let rhs = self.compile_expr(reg, *rhs);
					assert_eq!(rhs, reg);
					return reg;
				}

				let lhs = self.compile_expr(reg, *lhs);

				if Self::is_expr_constant(rhs.as_ref()) {
					let val = Self::compute_constant_expr(rhs.as_ref());

					let Literal::Int(val) = val else {panic!("Constant expression evaluation only support ints for now")};

					let opcode = match op {
						Operator::Add => Opcode::Addl,
						Operator::Sub => Opcode::Subl,
						Operator::Mul => Opcode::Mull,
						Operator::Div => Opcode::Divl,
						Operator::Lt => Opcode::Ltl,
						x => todo!("operation {x} not yet handled (literal)")
					};

					self.assembler.emit_opcode(opcode);
					self.assembler.emit_u8(reg);
					self.assembler.emit_u8(lhs);
					self.assembler.emit_i64(val);

				// let instr = match_infix_op_lit!(op, lhs, val, reg; (Add,Addl), (Mul,Mull), (Sub,Subl), (Div,Divl), (Lt, Ltl));

				// self.assembler.add_instr(instr);

				// TODO: constant lhs
				} else {
					let other_reg = self.env.allocate_reg();
					let rhs = self.compile_expr(other_reg, *rhs);

					// TODO: handle type checking
					// TODO: handle other ops

					let opcode = match op {
						Operator::Add => Opcode::Add,
						Operator::Sub => Opcode::Sub,
						Operator::Mul => Opcode::Mul,
						Operator::Div => Opcode::Div,
						Operator::Lt => Opcode::Lt,
						x => todo!("operation {x} not yet handled (literal)")
					};

					self.assembler.emit_opcode(opcode);
					self.assembler.emit_u8(reg);
					self.assembler.emit_u8(lhs);
					self.assembler.emit_u8(rhs);

					self.env.free_last_reg();
				}
				reg
			}
			Expr::FnNamedCall { name, args } => {
				if name == *"print" {
					// should be temporary, will be removed when proper std functions will be added
					let arg = args.into_iter().next().unwrap();
					let reg = self.compile_expr(reg, arg);

					self.assembler.emit_opcode(Opcode::Print); // TODO: multiple regs
					self.assembler.emit_u8(reg);
					return reg;
				} else if name == *"clock" {
					self.assembler.emit_opcode(Opcode::Clock);
					self.assembler.emit_u8(reg);
					return reg;
				}

				let f = self.env.get_function(&name); // TODO: handle functions declared after
				let arg_count = u8::try_from(args.len()).expect("Only accept up to 256 arguments");

				self.assembler.emit_opcode(Opcode::LoadF);
				self.assembler.emit_u8(reg);
				self.assembler.emit_u16(f);
				for (i, arg) in args.into_iter().enumerate() {
					#[allow(clippy::cast_possible_truncation)]
					let i = i as Reg;
					self.compile_expr(reg + i + 1, arg);
				}

				// TODO: handle multiple return values
				self.assembler.emit_opcode(Opcode::Call);
				self.assembler.emit_u8(reg);
				self.assembler.emit_u8(arg_count);
				self.assembler.emit_u8(1);

				reg
			}
			Expr::Block(_)
			| Expr::Error
			| Expr::Prefix(_, _)
			| Expr::FnCall { expr: _, args: _ } => unimplemented!()
		}
	}

	fn compile_let(&mut self, name: String, ty: Option<Ty>, val: Expr) {
		let ty = ty.unwrap().into();

		let reg = self.env.add_var(name, ty);

		if Self::is_expr_constant(&val) {
			let val = Self::compute_constant_expr(&val);
			self.load_lit(reg, val);
		} else {
			let new_reg = self.compile_expr(reg, val);
			if new_reg != reg {
				self.assembler.emit_opcode(Opcode::Move);
				self.assembler.emit_u8(reg); // dst
				self.assembler.emit_u8(new_reg);
			}
		}
	}

	fn load_lit(&mut self, reg: u8, lit: Literal) {
		match lit {
			Literal::Bool(x) => {
				self.assembler.emit_opcode(if x {
					Opcode::LoadTrue
				} else {
					Opcode::LoadFalse
				});
				self.assembler.emit_u8(reg);
			}
			Literal::Int(x) => {
				self.assembler.emit_opcode(Opcode::Load);
				self.assembler.emit_u8(reg);
				self.assembler.emit_i64(x);
			}
			Literal::Float(x) => {
				self.assembler.emit_opcode(Opcode::LoadFloat);
				self.assembler.emit_u8(reg);
				self.assembler.emit_f64(x);
			}
			Literal::String(_) => todo!()
		};
	}

	fn is_expr_constant(expr: &Expr) -> bool {
		match expr {
			Expr::Ident(_) => false, // TODO: check if the val of the ident (fn or variable) is constant
			Expr::Lit(_) => true,
			Expr::Infix { op: _, lhs, rhs } => {
				Self::is_expr_constant(lhs.as_ref()) && Self::is_expr_constant(rhs.as_ref())
			}
			Expr::Prefix(_, expr) => Self::is_expr_constant(expr.as_ref()),
			Expr::FnNamedCall { name: _, args: _ } => false, //unimplemented!(),
			Expr::FnCall { expr: _, args: _ } => false,      //unimplemented!(),
			Expr::Block(_) => false,                         //unimplemented!(),
			Expr::Error => false                             //unreachable!()
		}
	}

	fn compute_constant_expr(expr: &Expr) -> Literal {
		match expr {
			Expr::Lit(x) => x.clone(),
			Expr::Prefix(prefix, expr) => Self::compute_constant_prefix(*prefix, expr.as_ref()),
			Expr::Infix { op, lhs, rhs } => {
				Self::compute_constant_infix(*op, lhs.as_ref(), rhs.as_ref())
			}
			Expr::Block(_)
			| Expr::FnCall { expr: _, args: _ }
			| Expr::FnNamedCall { name: _, args: _ } => todo!(),
			Expr::Error | Expr::Ident(_) => unreachable!()
		}
	}

	fn compute_constant_prefix(prefix: Prefix, expr: &Expr) -> Literal {
		let val = Self::compute_constant_expr(expr);
		match prefix {
			Prefix::BitNot => {
				if let Literal::Int(x) = val {
					Literal::Int(!x) // ! if bitwise not for numbers in rust
				} else {
					panic!("Prefix 'bitnot' can only be applied to integers")
				}
			}
			Prefix::Not => {
				if let Literal::Bool(x) = val {
					Literal::Bool(!x)
				} else {
					panic!("Prefix 'not' can only be applied to bools")
				}
			}
			Prefix::Plus => {
				assert!(matches!(val, Literal::Float(_) | Literal::Int(_)));
				val
			}
			Prefix::Minus => {
				if let Literal::Int(x) = val {
					Literal::Int(-x)
				} else if let Literal::Float(x) = val {
					Literal::Float(-x)
				} else {
					panic!("Prefix 'minus' can only be applied to numbers")
				}
			}
			Prefix::Err => unreachable!()
		}
	}

	fn compute_constant_infix(op: Operator, lhs: &Expr, rhs: &Expr) -> Literal {
		let lhs = Self::compute_constant_expr(lhs);
		let rhs = Self::compute_constant_expr(rhs);
		match op {
			Operator::Add => lhs + rhs,
			Operator::Sub => lhs - rhs,
			Operator::Mul => lhs * rhs,
			Operator::Div => lhs / rhs,
			Operator::And => {
				assert!(matches!(lhs, Literal::Bool(_)));
				assert!(matches!(rhs, Literal::Bool(_)));
				todo!() //lhs & rhs
			}
			Operator::BitAnd => {
				assert!(matches!(lhs, Literal::Int(_) | Literal::Float(_)));
				assert!(matches!(rhs, Literal::Int(_) | Literal::Float(_)));
				todo!() //lhs & rhs
			}
			Operator::BitOr => todo!(),    //lhs | rhs,
			Operator::BitXor => todo!(),   //lhs ^ rhs,
			Operator::Exponent => todo!(), //lhs.pow(rhs),
			Operator::Gt => Literal::Bool(lhs > rhs),
			Operator::Gte => Literal::Bool(lhs >= rhs),
			Operator::Lt => Literal::Bool(lhs < rhs),
			Operator::Lte => Literal::Bool(lhs <= rhs),
			Operator::Eq => Literal::Bool(lhs == rhs),
			Operator::Neq => Literal::Bool(lhs != rhs),
			Operator::LShift => todo!(), //lhs << rhs,
			Operator::RShift => todo!(), //lhs >> rhs,
			Operator::Rem => todo!(),    //lhs % rhs,
			_ => unreachable!()
		}
	}

	fn compile_stmt(&mut self, stmt: Stmt) {
		match stmt {
			Stmt::Local { name, ty, val } => self.compile_let(name, ty, *val),
			Stmt::Expr(expr) => {
				if true {
					// !self.is_expr_constant(&expr) {
					let reg = self.env.allocate_reg();
					self.compile_expr(reg, expr);
					self.env.free_last_reg();
				}
			}
			Stmt::Item(item) => self.compile_item(item),
			Stmt::Error => unreachable!(),
			Stmt::Return(expr) | Stmt::FnReturn(expr) => {
				let reg = self.env.allocate_reg();
				let reg = self.compile_expr(reg, expr);

				self.assembler.program.returned = true;

				self.assembler.emit_opcode(Opcode::Ret);
				self.assembler.emit_u8(reg);
				self.assembler.emit_u8(1);
			}
			Stmt::If { cond, block } => {
				let reg = self.env.allocate_reg();
				self.compile_expr(reg, cond);
				self.assembler.emit_opcode(Opcode::JmpIfFalse);
				self.assembler.emit_u8(reg);
				let jmp = self.assembler.emit_u16(u16::MAX);
				self.env.free_last_reg();

				self.compile_block(block);
				let len = Address::try_from(self.assembler.program.code.len())
					.expect("Address bigger than maximum allowed"); // TODO: change that

				self.assembler.set_u16(jmp, len);
			}
			Stmt::While { cond, block } => {
				let while_start = Address::try_from(self.assembler.program.code.len())
					.expect("Address bigger than maximum allowed");
				let reg = self.env.allocate_reg();
				self.compile_expr(reg, cond);
				self.assembler.emit_opcode(Opcode::JmpIfFalse);
				self.assembler.emit_u8(reg);

				let jmp = self.assembler.emit_u16(u16::MAX);
				self.env.free_last_reg();

				self.compile_block(block);
				self.assembler.emit_opcode(Opcode::Jmp);
				self.assembler.emit_u16(while_start);
				let len = Address::try_from(self.assembler.program.code.len())
					.expect("Address bigger than maximum allowed"); // TODO: change that

				self.assembler.set_u16(jmp, len);
			}
		}
	}

	fn compile_function(&mut self, name: String, args: Vec<Argument>, _ty: Ty, block: Vec<Stmt>) {
		let mut f = Self::new();
		let i = u16::try_from(self.assembler.program.functions.len())
			.expect("More than 2^16 - 1 (u16) functions");

		f.env.set_function(name.clone(), i);
		self.env.set_function(name, i);
		for arg in args {
			f.env.add_var(arg.name, arg.ty.into());
		}

		f.compile(block);

		if !f.assembler.program.returned {
			f.assembler.emit_opcode(Opcode::Ret);
			f.assembler.emit_u8(0);
			f.assembler.emit_u8(0);
		}

		self.assembler.add_function(f.assembler.program);
	}

	fn compile_item(&mut self, item: Item) {
		match item {
			Item::Function {
				name,
				args,
				ty,
				block
			} => self.compile_function(name, args, ty, block),
			_ => todo!()
		}
	}

	fn compile_block(&mut self, block: Vec<Stmt>) {
		for stmt in block {
			self.compile_stmt(stmt);
		}
	}

	pub fn compile(&mut self, block: Vec<Stmt>) -> Program {
		self.compile_block(block);
		self.assembler.emit_opcode(Opcode::Halt);
		self.assembler.program.clone()
	}

	pub fn new() -> Self {
		Self {
			assembler: Assembler::new(),
			env: Env::default()
		}
	}
}
