pub mod ast;
mod expressions;
mod item;
mod statements;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::ast::{Block, Operator, ParseError, Ty};

pub type RetItem = (Result<Token, ()>, Range<usize>);
pub type IteratorItem = (Token, Range<usize>);

pub struct Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	tokens: Peekable<I>,
	source: &'a str,
	range: Range<usize>,
	// phase: ,
	errors: Vec<(ParseError, Range<usize>)>,
	allow_implicit_types: bool // TODO: fn should_skip_token
}

impl<'a> Parser<'a, SpannedIter<'a, Token>> {
	pub fn new(
		source: &'a str /*, allow_implicit_types: bool*/
	) -> Parser<'a, SpannedIter<'a, Token>> {
		let lex = Token::lexer(source);
		Self {
			tokens: lex.spanned().peekable(),
			source,
			range: 0..0,
			errors: Vec::new(),
			allow_implicit_types: false
		}
	}
}

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn unwrap_ref(t: Option<&RetItem>) -> Option<IteratorItem> {
		match t {
			Some(&(Ok(ref token), ref r)) => Some((*token, r.clone())),
			_ => None
		}
	}

	const fn unwrap(t: Option<RetItem>) -> Option<IteratorItem> {
		match t {
			Some((Ok(token), r)) => Some((token, r)),
			_ => None
		}
	}

	fn peek(&mut self) -> Option<Token> {
		let next = self.peek_range();
		if let Some((t, _)) = next {
			Some(t)
		} else {
			None
		}
	}

	fn peek_ignore(&mut self, ignore: Token) -> Option<Token> {
		let mut peek = self.peek();
		while peek == Some(ignore) {
			self.next();
			peek = self.peek();
		}
		peek
	}

	fn peek_range(&mut self) -> Option<(Token, Range<usize>)> {
		Parser::<'a, I>::unwrap_ref(self.tokens.peek())
	}

	fn next(&mut self) -> Option<Token> {
		let next = Parser::<'a, I>::unwrap(self.tokens.next());
		if let Some((t, r)) = next {
			self.range = r;
			Some(t)
		} else {
			None
		}
	}

	fn at(&mut self, expected: Token) -> bool {
		self.peek().map_or(false, |token| token == expected)
	}

	fn text(&self) -> String {
		self.source[self.range.clone()].to_string()
	}

	fn consume(&mut self, expected: Token) {
		let Some(next) = self.next() else {
			return self.push_error(ParseError::ExpectedTokenButNotFound(expected));
		};

		if next != expected {
			self.push_error(ParseError::ExpectedTokenButFoundInstead {
				expected,
				found: next
			});
		}
		//self.consume_raw(expected, false);
	}

	/*
		fn consume_raw(&mut self, expected: Token, consume_if_error: bool) {
			match self.peek_range() {
				Some((token, range)) => {
					if token == expected {
						self.next();
					} else {
						self.errors.push((
							ParseError::ExpectedTokenButFoundInstead {
								expected,
								found: token
							},
							range
						));
						if consume_if_error {
							self.next();
						}
					}
				}
				None => self.push_error(ParseError::ExpectedTokenButNotFound(expected))
			}
		}
	*/
	// utils

	const fn is_lit(token: Token) -> bool {
		matches!(
			token,
			Token::String | Token::Int | Token::Float | Token::True | Token::False
		)
	}

	fn is_ident(token: Token) -> bool {
		token == Token::Identifier
	}

	const fn is_op(token: Token) -> bool {
		// TODO: expand that
		matches!(
			token,
			Token::Plus
				| Token::PlusEq | Token::Minus
				| Token::MinusEq | Token::Asterisk
				| Token::AsteriskEq
				| Token::Slash | Token::SlashEq
				| Token::Percent | Token::PercentEq
				| Token::Anpersand
				| Token::ExclamationMark
				| Token::AnpersandEq
				| Token::Bar | Token::BarEq
				| Token::Caret | Token::CaretEq
				| Token::LShift | Token::LShiftEq
				| Token::RShift | Token::RShiftEq
				| Token::Gte | Token::Lte
				| Token::LChevron
				| Token::RChevron
				| Token::Eq | Token::Neq
				| Token::And | Token::AndEq
				| Token::Or | Token::OrEq
				| Token::Tilde | Token::TildeEq
				| Token::DoubleEq
		)
	}

	const fn is_keyword(token: Token) -> bool {
		matches!(
			token,
			Token::Fn | Token::Let | Token::If | Token::For | Token::While | Token::Return
		)
	}

	// TODO: detect with visibility (pub)
	const fn is_item_start(token: Token) -> bool {
		matches!(token, Token::Fn | Token::Const | Token::Struct)
	}

	fn get_ident(&mut self) -> String {
		let Some(ident) = self.peek() else {
			self.push_error(ParseError::UnexpectedEOF);
			return String::new()
		};
		if ident == Token::Identifier {
			self.next();
			self.text()
		} else {
			self.push_error(ParseError::ExpectedTokenButFoundInstead {
				expected: Token::Identifier,
				found: ident
			});
			String::new() // TODO: error message in string ?
		}
	}

	const fn operator_precedence(op: Operator) -> usize {
		// https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages
		type Op = Operator;
		match op {
			Op::Not | Op::BitNot => 13,
			Op::Exponent => 12,
			Op::Mul | Op::Div | Op::Rem => 11,
			Op::Add | Op::Sub => 10,
			Op::LShift | Op::RShift => 9,
			Op::Lt | Op::Lte | Op::Gt | Op::Gte => 8,
			Op::Eq | Op::Neq => 7,
			Op::BitAnd => 6,
			Op::BitXor => 5,
			Op::BitOr => 4,
			Op::And => 3,
			Op::Or => 2,
			Op::Assign
			| Op::AddEq
			| Op::SubEq
			| Op::MulEq
			| Op::DivEq
			| Op::RemEq
			| Op::BitAndEq
			| Op::BitOrEq
			| Op::BitXorEq
			| Op::LShiftEq
			| Op::RShiftEq => 1,
			_ => 0
		}
	}

	#[allow(clippy::range_plus_one)]
	const fn eof_range(&self) -> Range<usize> {
		let len = self.source.len();
		len..len + 1
	}

	fn is_eof(&mut self) -> bool {
		self.peek().is_none()
	}

	fn push_error(&mut self, error: ParseError) {
		let range = if error == ParseError::UnexpectedEOF {
			self.eof_range()
		} else {
			self.range.clone()
		};
		self.errors.push((error, range));
	}

	fn parse_l<T, F: Fn(&mut Parser<'a, I>) -> T>(&mut self, end_token: Token, f: F) -> Vec<T> {
		let mut args = Vec::new();
		loop {
			match self.peek() {
				None => {
					self.push_error(ParseError::UnexpectedEOF);
					break;
				}
				Some(x) => {
					if x == end_token || x == Token::SemiColon {
						break;
					}
				}
			}

			args.push(f(self));

			if !self.at(end_token) {
				self.consume(Token::Comma);
			} else if self.at(Token::Comma) {
				// trailing comma
				self.next();
			}
			if self.is_eof() {
				self.push_error(ParseError::UnexpectedEOF);
				break;
			}
		}
		args
	}

	fn parse_ty(&mut self) -> Ty {
		// TODO: change that
		let name = self.get_ident();
		Ty::Ident(name)
	}

	//

	#[cfg(test)]
	pub const fn errors(&self) -> &Vec<(ParseError, Range<usize>)> {
		&self.errors
	}

	pub fn parse(&mut self) -> (Block, &Vec<(ParseError, Range<usize>)>) {
		let parsed = self.parse_block();

		#[cfg(test)]
		if !self.errors.is_empty() {
			eprintln!("\n\nparse errors: {:?}\n\n", &self.errors);
		}

		(parsed, &self.errors)
	}
}
