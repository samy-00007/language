use crate::lexer::Token;
use language_ast::Expr;

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