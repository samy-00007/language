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

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
	Ident(String),
	Lit(Literal),
	Prefix(Prefix, E),
	Infix { op: Operator, lhs: E, rhs: E },
	Block(Block), // FIXME: handle statements in there
	FnCall { expr: E, args: Vec<Expr> },
	FnNamedCall { name: String, args: Vec<Expr> }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
	Int(i128),
	Float(f128), // TODO: test that
	Bool(bool),
	String(String)
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Prefix {
	Not // bitwise invert
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Operator {
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

impl From<Token> for Operator {
	fn from(value: Token) -> Self {
		match value {
			Token::Plus => Operator::Add,
			Token::PlusEq => Operator::AddEq,
			Token::Minus => Operator::Sub,
			Token::MinusEq => Operator::SubEq,
			Token::Asterisk => Operator::Mul,
			Token::AsteriskEq => Operator::MulEq,
			Token::DoubleAsterisk => Operator::Exponent,
			Token::DoubleAsteriskEq => Operator::ExponentEq,
			Token::Slash => Operator::Div,
			Token::SlashEq => Operator::DivEq,
			Token::Percent => Operator::Rem,
			Token::PercentEq => Operator::RemEq,
			Token::ExclamationMark => Operator::Not,

			Token::Tilde => Operator::BitNot,
			Token::TildeEq => Operator::BitNotEq,
			Token::Anpersand => Operator::BitAnd,
			Token::AnpersandEq => Operator::BitAndEq,
			Token::Bar => Operator::BitOr,
			Token::BarEq => Operator::BitOrEq,
			Token::Caret => Operator::BitXor,
			Token::CaretEq => Operator::BitXorEq,
			Token::LShift => Operator::LShift,
			Token::LShiftEq => Operator::LShiftEq,
			Token::RShift => Operator::RShift,
			Token::RShiftEq => Operator::RShiftEq,

			Token::Eq => Operator::Eq,
			Token::Gte => Operator::Gte,
			Token::Lte => Operator::Lte,
			Token::Neq => Operator::Neq,
			Token::And => Operator::And,
			Token::AndEq => Operator::AndEq,
			Token::Or => Operator::Or,
			Token::OrEq => Operator::OrEq,
			Token::RChevron => Operator::Gt,
			Token::LChevron => Operator::Lt,
			_ => panic!(
				"Unexpected token while converting to operator: '{:?}'",
				value
			)
		}
	}
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ParseError {
	UnexpectedEOF,
	UnexpectedToken(Token), // TODO: maybe store the token text ?
	ExpectedTokenButFoundInstead(Token, Token),
	ExpectedTokenButNotFound(Token),
	ExpectedExprButFoundInstead(Expr, Expr),
	ExpectedExprButNotFound(Expr)
}

impl std::fmt::Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(
			f,
			"{}",
			match self {
				Self::UnexpectedEOF => "Expected expression but found <EOF>".to_string(),
				Self::UnexpectedToken(t) => format!("Unexpected token '{t:?}' found"),
				Self::ExpectedTokenButFoundInstead(a, b) =>
					format!("Expected token '{a:?}' but found '{b:?}' instead"),
				Self::ExpectedTokenButNotFound(t) => format!("Expected token '{t:?}'"),
				Self::ExpectedExprButFoundInstead(a, b) => format!("Expected expression '{a:?}' but found '{b:?}' instead"),
				Self::ExpectedExprButNotFound(t) => format!("Expected expression '{t:?}'")
			}
		)
	}
}
