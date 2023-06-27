use crate::lexer::Token;
use language_ast::Expr;
use std::fmt::Display;

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
