use language_ast::Ty;

#[derive(Debug)]
pub struct Var {
	pub reg: u8,
	pub ty: Type
}

impl Var {
	pub const fn new(reg: u8, ty: Type) -> Self {
		Self { reg, ty }
	}
}

#[derive(Debug)]
pub enum Type {
	Bool,
	Number,
	String
}

impl From<Ty> for Type {
	fn from(value: Ty) -> Self {
		let Ty::Ident(ty) = value else {unreachable!()}; // TODO: handle that
		match ty.as_str() {
			"string" => Self::String,
			"number" => Self::Number,
			"bool" => Self::Bool,
			_ => unreachable!()
		}
	}
}

#[macro_export]
macro_rules! match_infix_op {
	($op:ident, $reg_1:ident, $reg_2:ident, $dst:ident; $($name:ident),*) => {
		match $op {
			$(
				Operator::$name => Instr::$name {
					op_1: $reg_1,
					op_2: $reg_2,
					dst: $dst,
				},
			)*
			x => todo!("Operator {x:?} not yet handled")
		}
	};
}

#[macro_export]
macro_rules! match_infix_op_lit {
	($op:ident, $reg_1:ident, $reg_2:ident, $dst:ident; $(($op_name:ident, $name:ident)),*) => {
		match $op {
			$(
				Operator::$op_name => Instr::$name {
					op_1: $reg_1,
					op_2: $reg_2,
					dst: $dst,
				},
			)*
			x => todo!("Operator {x:?} not yet handled")
		}
	};
}
