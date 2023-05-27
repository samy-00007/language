use std::fmt::Display;

//use f128::f128;

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
		val: E,
	},
	Function {
		name: String,
		args: Vec<(String, String)>,
		t: String,
		block: Block,
	},
	If {
		cond: Expr,
		block: Block,
	},
	While {
		cond: Expr,
		block: Block,
	},
	Return(Expr),
	Expr(Expr),
	FnReturn(Expr),
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
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Literal {
	Int(i128),
	Float(f64), // TODO: test that
	Bool(bool),
	String(String),
}


#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Prefix {
	Not, // bitwise invert
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
	OrEq,
}

impl Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Not => "~",
		};
		write!(f, "{res}")
	}
}

impl From<Token> for Operator {
	fn from(value: Token) -> Self {
		match value {
			Token::Eq => Self::Assign,

			Token::Plus => Self::Add,
			Token::PlusEq => Self::AddEq,
			Token::Minus => Self::Sub,
			Token::MinusEq => Self::SubEq,
			Token::Asterisk => Self::Mul,
			Token::AsteriskEq => Self::MulEq,
			Token::DoubleAsterisk => Self::Exponent,
			Token::DoubleAsteriskEq => Self::ExponentEq,
			Token::Slash => Self::Div,
			Token::SlashEq => Self::DivEq,
			Token::Percent => Self::Rem,
			Token::PercentEq => Self::RemEq,
			Token::ExclamationMark => Self::Not,

			Token::Tilde => Self::BitNot,
			Token::TildeEq => Self::BitNotEq,
			Token::Anpersand => Self::BitAnd,
			Token::AnpersandEq => Self::BitAndEq,
			Token::Bar => Self::BitOr,
			Token::BarEq => Self::BitOrEq,
			Token::Caret => Self::BitXor,
			Token::CaretEq => Self::BitXorEq,
			Token::LShift => Self::LShift,
			Token::LShiftEq => Self::LShiftEq,
			Token::RShift => Self::RShift,
			Token::RShiftEq => Self::RShiftEq,

			Token::DoubleEq => Self::Eq,
			Token::Gte => Self::Gte,
			Token::Lte => Self::Lte,
			Token::Neq => Self::Neq,
			Token::And => Self::And,
			Token::AndEq => Self::AndEq,
			Token::Or => Self::Or,
			Token::OrEq => Self::OrEq,
			Token::RChevron => Self::Gt,
			Token::LChevron => Self::Lt,
			_ => unreachable!(),
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
	ExpectedExprButNotFound(Expr),
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Bool(x) => x.to_string(),
			Self::Float(x) => x.to_string(),
			Self::Int(x) => x.to_string(),
			Self::String(x) => format!("\"{x}\""),
		};
		write!(f, "{res}")
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Assign => "=",

			Self::Add => "+",
			Self::AddEq => "+=",
			Self::Sub => "-",
			Self::SubEq => "-=",
			Self::Mul => "*",
			Self::MulEq => "*=",
			Self::Exponent => "**",
			Self::ExponentEq => "**=",
			Self::Div => "/",
			Self::DivEq => "/=",
			Self::Rem => "%",
			Self::RemEq => "%=",
			Self::Not => "!",

			Self::BitAnd => "&",
			Self::BitAndEq => "&=",
			Self::BitOr => "|",
			Self::BitOrEq => "|=",
			Self::BitNot => "~",
			Self::BitNotEq => "~=",
			Self::BitXor => "^",
			Self::BitXorEq => "^=",
			Self::LShift => "<<",
			Self::LShiftEq => "<<=",
			Self::RShift => ">>",
			Self::RShiftEq => ">>=",

			Self::Eq => "==",
			Self::Gt => ">",
			Self::Gte => ">=",
			Self::Lt => "<",
			Self::Lte => "<=",
			Self::Neq => "!=",
			Self::And => "&&",
			Self::AndEq => "&&=",
			Self::Or => "||",
			Self::OrEq => "||=",
		};

		write!(f, "{res}")
	}
}

impl Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Expr(x) => format!("{x};"),
			Self::FnReturn(x) => format!("return {x}"),
			Self::If { cond, block } => format!("if ({}) {{\n{}\n}}", cond, print_s(block, "\n")),
			Self::Local { name, t, val } => {
				let t_ = t.as_ref().map_or_else(String::new, |t| format!(": {}", t));
				format!("let {name}{t_} = {val};")
			}
			Self::Return(x) => format!("{x}"),
			Self::Function {
				name,
				args,
				t,
				block,
			} => format!(
				"fn {}({}): {} {{\n{}\n}}",
				name,
				args.iter()
					.map(|x| format!("{}: {}", x.0, x.1))
					.collect::<Vec<String>>()
					.join("\n"),
				t,
				print_s(block, "\n")
			),
			Self::While { cond, block } => {
				format!("while ({}) {{\n{}\n}}", cond, print_s(block, "\n"))
			}
		};
		write!(f, "{res}")
	}
}

fn print_s<T>(vec: &[T], sep: &str) -> String
where
	T: Display,
{
	vec.iter()
		.map(std::string::ToString::to_string)
		.collect::<Vec<String>>()
		.join(sep)
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Block(x) => format!("{{\n{}\n}}", print_s(x, "\n")),
			Self::FnNamedCall { name, args } => format!("{name}({})", print_s(args, ", ")),
			Self::Ident(s) => s.to_string(),
			Self::Lit(l) => format!("{l}"),
			Self::Infix { op, lhs, rhs } => format!("({lhs} {op} {rhs})"),
			Self::Prefix(prefix, e) => format!("({prefix}{e})"),
			Self::FnCall { expr, args } => format!("{}({})", expr, print_s(args, ", ")),
		};
		write!(f, "{res}")
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::UnexpectedEOF => "Expected expression but found <EOF>".to_string(),
			Self::UnexpectedToken(t) => format!("Unexpected token '{t:?}' found"),
			Self::ExpectedTokenButFoundInstead(a, b) => {
				format!("Expected token '{a:?}' but found '{b:?}' instead")
			}
			Self::ExpectedTokenButNotFound(t) => format!("Expected token '{t:?}'"),
			Self::ExpectedExprButFoundInstead(a, b) => {
				format!("Expected expression '{a:?}' but found '{b:?}' instead")
			}
			Self::ExpectedExprButNotFound(t) => format!("Expected expression '{t:?}'"),
		};
		write!(f, "{res}")
	}
}
