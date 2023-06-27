use std::ops::{Add, Div, Mul, Sub};

//use f128::f128;

// https://github.com/Rydgel/monkey-rust/blob/master/lib/parser/ast.rs

type E = Box<Expr>;
pub type Block = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ty {
	Ident(String),
	None
}

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
	Function {
		name: String,
		args: Vec<Argument>,
		ty: Ty,
		block: Block
	},
	Struct {
		name: String,
		fields: Vec<Argument>
	},
	Constant {
		name: String,
		ty: Ty,
		value: Expr
	} // TODO: module, use, impl, enum
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Item(Item),
	Local {
		// var decl
		name: String,
		ty: Option<Ty>,
		val: E
	},
	// Function {
	// 	name: String,
	// 	generics: Vec<Generic>,
	// 	args: Vec<Argument>,
	// 	t: Option<String>,
	// 	block: Block
	// },
	If {
		cond: Expr,
		block: Block
	},
	While {
		cond: Expr,
		block: Block
	},
	Return(Expr),
	Expr(Expr),
	FnReturn(Expr),
	Error
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Ident(String),
	Lit(Literal),
	Prefix(Prefix, E),
	Infix { op: Operator, lhs: E, rhs: E },
	Block(Block), // FIXME: handle statements in there
	FnCall { expr: E, args: Vec<Expr> },
	FnNamedCall { name: String, args: Vec<Expr> },
	Error
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
	Int(i64),
	Float(f64), // TODO: test that
	Bool(bool),
	String(String)
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Prefix {
	Not,    // boolean invert
	BitNot, // bitwise invert
	Plus,
	Minus,
	Err
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Operator {
	Assign,

	Add,
	AddEq,
	Sub,
	SubEq,
	Mul,
	MulEq,
	Exponent,
	ExponentEq,
	Div,
	DivEq,
	Rem,
	RemEq,
	Not,

	BitAnd,
	BitAndEq,
	BitOr,
	BitOrEq,
	BitNot,
	BitNotEq,
	BitXor,
	BitXorEq,
	LShift,
	LShiftEq,
	RShift,
	RShiftEq,

	Eq,
	Gt,
	Gte,
	Lt,
	Lte,
	Neq,
	And,
	AndEq,
	Or,
	OrEq
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Generic {
	pub name: String,
	pub traits: Vec<String>
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Argument {
	pub name: String,
	pub ty: Ty
}

impl TryFrom<Operator> for Prefix {
	type Error = Operator;
	fn try_from(value: Operator) -> Result<Self, Self::Error> {
		Ok(match value {
			Operator::Add => Self::Plus,
			Operator::Sub => Self::Minus,
			Operator::BitNot => Self::BitNot,
			Operator::Not => Self::Not,
			_ => return Err(value)
		})
	}
}

macro_rules! literal_op {
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

literal_op!(Add, add, +, false);
literal_op!(Sub, sub, -, false);
literal_op!(Mul, mul, -, false);
literal_op!(Div, div, /, true);
