use f128::f128;

use crate::lexer::Token;
// https://github.com/Rydgel/monkey-rust/blob/master/lib/parser/ast.rs

type E = Box<Expr>;
pub(super) type Block = Vec<Stmt>;

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
	Local {
		// var decl
		name: String,
		t: Option<String>,
		val: E
	},
	Function {
		name: String,
		args: Vec<(String, String)>,
		t: String,
		block: Block
	},
	If {
		cond: Expr,
		block: Block
	},
	Return(Expr),
	Expr(Expr)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
	Ident(String),
	Lit(Literal),
	Prefix(Prefix, E),
	Infix { op: Operator, lhs: E, rhs: E },
	Block(Block), // FIXME: handle statements in there
	FnCall { name: String, args: Vec<Expr> }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Int(u128),
	Float(f128), // TODO: test that
	Bool(bool),
	String(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Prefix {
	Not
	// bitwise invert
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
	Plus,
	Minus,
	Mul,
	Div,
	Eq //FIXME:
}

impl From<Token> for Operator {
	fn from(value: Token) -> Self {
		match value {
			Token::Plus => Operator::Plus,
			Token::Minus => Operator::Minus,
			Token::Asterisk => Operator::Mul,
			Token::Slash => Operator::Div,
			Token::Eq => Operator::Eq,
			_ => panic!(
				"Unexpected token while converting to operator: '{:?}'",
				value
			)
		}
	}
}
