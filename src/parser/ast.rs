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
	Expr(Expr),
	FnReturn(Expr)
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

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Prefix {
	Not
	// bitwise invert
}

#[derive(Debug, Clone, PartialEq, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
#[allow(dead_code)]
pub enum ParseError {
	UnexpectedEOF,
	UnexpectedToken(Token), // TODO: maybe store the token text ?
	ExpectedButFoundInstead(Token, Token),
	ExpectedButNotFound(Token)
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}",
			match self {
				Self::UnexpectedEOF => "Expected expression but found <EOF>".to_string(),
				Self::UnexpectedToken(t) => format!("Unexpected token '{t:?}' found"),
				Self::ExpectedButFoundInstead(a, b) => format!("Expected token '{a:?}' but found '{b:?}' instead"),
				Self::ExpectedButNotFound(t) => format!("Expected token '{t:?}'")
			}
		)
	}
}