use std::fmt::Display;

//use f128::f128;

use crate::lexer::Token;
// https://github.com/Rydgel/monkey-rust/blob/master/lib/parser/ast.rs

type E = Box<Expr>;
pub(super) type Block = Vec<Stmt>;

/*
#[derive(Eq, PartialEq, Clone, Copy, Hash, Default, Debug)]
pub struct Span {
	start: usize,
	end: usize
}

impl Index<Span> for str {
	type Output = str;
	fn index(&self, index: Span) -> &Self::Output {
		&self[Range::<usize>::from(index)]
	}
}

impl From<Span> for Range<usize> {
	fn from(value: Span) -> Self {
		(value.start)..(value.end)
	}
}

impl From<Range<usize>> for Span {
	fn from(value: Range<usize>) -> Self {
		Self { start: value.start, end: value.end }
	}
}
*/

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
		t: Ty,
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
		t: Option<Ty>,
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
	Int(i128),
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

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ParseError {
	UnexpectedEOF,
	UnexpectedToken(Token), // TODO: maybe store the token text ?
	ExpectedTokenButFoundInstead { expected: Token, found: Token },
	ExpectedTokenButNotFound(Token),
	ExpectedExprButFoundInstead { expected: Expr, found: Expr },
	ExpectedExprButNotFound(Expr),
	IntParseError(String),
	FloatParseError(String),
	NoImplicitTypeAllowed
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

//

impl Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Expr(x) => format!("{x};"),
			Self::FnReturn(x) => format!("return {x}"),
			Self::If { cond, block } => format!("if ({}) {{\n{}\n}}", cond, print_s(block, "\n")),
			Self::Local { name, t, val } => {
				let t_ = t
					.as_ref()
					.map_or_else(String::new, |t| format!(": {:?}", t));
				format!("let {name}{t_} = {val};")
			}
			Self::Return(x) => format!("{x}"),
			// Self::Function {
			// 	name,
			// 	generics,
			// 	args,
			// 	t,
			// 	block
			// } => format!(
			// 	"fn {}{}({}) {} {{\n{}\n}}",
			// 	name,
			// 	print_l(generics, ", ", "<", ">"),
			// 	args.iter()
			// 		.map(|x| format!("{}: {}", x.name, x.ty))
			// 		.collect::<Vec<String>>()
			// 		.join("\n"),
			// 	t.as_ref().map_or_else(String::new, |ty| format!("-> {}", ty)),
			// 	print_s(block, "\n")
			// ),
			Self::While { cond, block } => {
				format!("while ({}) {{\n{}\n}}", cond, print_s(block, "\n"))
			}
			Self::Error => "<STMT ERROR>".to_string(),
			_ => todo!()
		};
		write!(f, "{res}")
	}
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
			Self::Error => "<EXPR ERROR>".to_string()
		};
		write!(f, "{res}")
	}
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Bool(x) => x.to_string(),
			Self::Float(x) => x.to_string(),
			Self::Int(x) => x.to_string(),
			Self::String(x) => format!("\"{x}\"")
		};
		write!(f, "{res}")
	}
}

impl Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::BitNot => "~",
			Self::Not => "!",
			Self::Minus => "-",
			Self::Plus => "+",
			Self::Err => "<PREFIX ERROR>"
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
			Self::OrEq => "||="
		};

		write!(f, "{res}")
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::UnexpectedEOF => "Expected expression but found <EOF>".to_string(),
			Self::UnexpectedToken(t) => format!("Unexpected token '{t:?}' found"),
			Self::ExpectedTokenButFoundInstead {
				expected: a,
				found: b
			} => {
				format!("Expected token '{a:?}' but found '{b:?}' instead")
			}
			Self::ExpectedTokenButNotFound(t) => format!("Expected token '{t:?}'"),
			Self::ExpectedExprButFoundInstead {
				expected: a,
				found: b
			} => {
				format!("Expected expression '{a:?}' but found '{b:?}' instead")
			}
			Self::ExpectedExprButNotFound(t) => format!("Expected expression '{t:?}'"),
			Self::IntParseError(s) => format!("Could not parse '{s}' into an int"),
			Self::FloatParseError(s) => format!("Could not parse '{s}' into an float"),
			Self::NoImplicitTypeAllowed => {
				"Implicit type is not allowed, please explicit the type".into()
			}
		};
		write!(f, "{res}")
	}
}

impl Display for Generic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let traits = self.traits.join(" + ");
		let traits = if traits.is_empty() {
			traits
		} else {
			": ".to_string() + traits.as_str()
		};
		write!(f, "{}{}", self.name, traits)
	}
}

//

impl TryFrom<Operator> for Prefix {
	type Error = ParseError;
	fn try_from(value: Operator) -> Result<Self, Self::Error> {
		Ok(match value {
			Operator::Add => Self::Plus,
			Operator::Sub => Self::Minus,
			Operator::BitNot => Self::BitNot,
			Operator::Not => Self::Not,
			_ => return Err(ParseError::UnexpectedToken(value.into()))
		})
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
			_ => unreachable!()
		}
	}
}

impl From<Operator> for Token {
	fn from(value: Operator) -> Self {
		match value {
			Operator::Assign => Self::Eq,

			Operator::Add => Self::Plus,
			Operator::AddEq => Self::PlusEq,
			Operator::Sub => Self::Minus,
			Operator::SubEq => Self::MinusEq,
			Operator::Mul => Self::Asterisk,
			Operator::MulEq => Self::AsteriskEq,
			Operator::Exponent => Self::DoubleAsterisk,
			Operator::ExponentEq => Self::DoubleAsteriskEq,
			Operator::Div => Self::Slash,
			Operator::DivEq => Self::SlashEq,
			Operator::Rem => Self::Percent,
			Operator::RemEq => Self::PercentEq,
			Operator::Not => Self::ExclamationMark,

			Operator::BitNot => Self::Tilde,
			Operator::BitNotEq => Self::TildeEq,
			Operator::BitAnd => Self::Anpersand,
			Operator::BitAndEq => Self::AnpersandEq,
			Operator::BitOr => Self::Bar,
			Operator::BitOrEq => Self::BarEq,
			Operator::BitXor => Self::Caret,
			Operator::BitXorEq => Self::CaretEq,
			Operator::LShift => Self::LShift,
			Operator::LShiftEq => Self::LShiftEq,
			Operator::RShift => Self::RShift,
			Operator::RShiftEq => Self::RShiftEq,

			Operator::Eq => Self::DoubleEq,
			Operator::Gte => Self::Gte,
			Operator::Lte => Self::Lte,
			Operator::Neq => Self::Neq,
			Operator::And => Self::And,
			Operator::AndEq => Self::AndEq,
			Operator::Or => Self::Or,
			Operator::OrEq => Self::OrEq,
			Operator::Gt => Self::RChevron,
			Operator::Lt => Self::LChevron
		}
	}
}

fn print_l<T: Display>(vec: &[T], sep: &str, surround_l: &str, surround_r: &str) -> String {
	let s = print_s(vec, sep);
	if s.is_empty() {
		s
	} else {
		format!("{surround_l}{s}{surround_r}")
	}
}

fn print_s<T: Display>(vec: &[T], sep: &str) -> String {
	vec.iter()
		.map(std::string::ToString::to_string)
		.collect::<Vec<String>>()
		.join(sep)
}
