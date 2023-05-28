pub mod ast;
mod expressions;
mod statements;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::ast::{Block, ParseError, Operator};

pub type RetItem = (Result<Token, ()>, Range<usize>);
pub type Item = (Token, Range<usize>);

pub struct Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	tokens: Peekable<I>,
	source: &'a str,
	range: Range<usize>,
	// phase: ,
	errors: Vec<(ParseError, Range<usize>)>
}

impl<'a> Parser<'a, SpannedIter<'a, Token>> {
	pub fn new(source: &'a str) -> Parser<'a, SpannedIter<'a, Token>> {
		let lex = Token::lexer(source);
		Self {
			tokens: lex.spanned().peekable(),
			source,
			range: 0..0,
			errors: Vec::new()
		}
	}
}

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn unwrap_ref(t: Option<&RetItem>) -> Option<Item> {
		match t {
			Some(&(Ok(ref token), ref r)) => Some((*token, r.clone())),
			_ => None
		}
	}

	const fn unwrap(t: Option<RetItem>) -> Option<Item> {
		match t {
			Some((Ok(token), r)) => Some((token, r)),
			_ => None
		}
	}

	pub(self) fn peek(&mut self) -> Option<Token> {
		let next = self.peek_range();
		if let Some((t, _)) = next {
			Some(t)
		} else {
			None
		}
	}

	fn peek_range(&mut self) -> Option<(Token, Range<usize>)> {
		Parser::<'a, I>::unwrap_ref(self.tokens.peek())
	}

	pub(self) fn next(&mut self) -> Option<Token> {
		let next = Parser::<'a, I>::unwrap(self.tokens.next());
		if let Some((t, r)) = next {
			self.range = r;
			Some(t)
		} else {
			None
		}
	}

	pub(self) fn at(&mut self, expected: Token) -> bool {
		self.peek().map_or(false, |token| token == expected)
	}

	pub(self) fn text(&self) -> String {
		self.source[self.range.clone()].to_string()
	}

	pub(self) fn consume(&mut self, expected: Token) {
		self.consume_raw(expected, false);
	}

	pub(self) fn consume_raw(&mut self, expected: Token, consume_if_error: bool) {
		match self.peek_range() {
			Some((token, range)) => {
				if token == expected {
					self.next();
				} else {
					self.errors.push((ParseError::ExpectedTokenButFoundInstead(expected, token), range));
					if consume_if_error {
						self.next();
					}
				}
			},
			None => self.push_error(ParseError::ExpectedTokenButNotFound(expected))
		}
	}

	// utils

	pub(self) const fn is_lit(token: Token) -> bool {
		matches!(
			token,
			Token::String | Token::Int | Token::Float | Token::True | Token::False
		)
	}

	pub(self) fn is_ident(token: Token) -> bool {
		token == Token::Identifier
	}

	pub(self) const fn is_op(token: Token) -> bool {
		// TODO: expand that
		matches!(
			token,
			Token::Plus
				| Token::PlusEq | Token::Minus
				| Token::MinusEq | Token::Asterisk
				| Token::AsteriskEq
				| Token::Slash | Token::SlashEq
				| Token::Percent | Token::PercentEq
				| Token::Anpersand | Token::ExclamationMark
				| Token::AnpersandEq
				| Token::Bar | Token::BarEq
				| Token::Caret | Token::CaretEq
				| Token::LShift | Token::LShiftEq
				| Token::RShift | Token::RShiftEq
				| Token::Gte | Token::Lte
				| Token::LChevron | Token::RChevron
				| Token::Eq | Token::Neq
				| Token::And | Token::AndEq
				| Token::Or | Token::OrEq
				| Token::Tilde | Token::TildeEq
				| Token::DoubleEq
		)
	}

	pub(self) const fn is_keyword(token: Token) -> bool {
		matches!(
			token,
			Token::Fn | Token::Let | Token::If | Token::For | Token::While | Token::Return
		)
	}

	pub(self) fn get_ident(&mut self) -> String {
		let Some(ident) = self.peek() else {
			self.push_error(ParseError::UnexpectedEOF);
			return String::new()
		};
		if ident == Token::Identifier {
			self.next();
			self.text()
		} else {
			self.push_error(ParseError::ExpectedTokenButFoundInstead(
				Token::Identifier,
				ident
			));
			String::new() // TODO: error message in string ?
		}
	}

	pub(self) const fn operator_precedence(op: Operator) -> usize {
		// https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages
		type Op = Operator;
		match op {
			Op::Not | Op::BitNot => 13,
			Op::Exponent => 12,
			Op::Mul | Op::Div | Op::Rem => 11,
			Op::Add | Op::Sub  => 10,
			Op::LShift | Op::RShift => 9,
			Op::Lt | Op::Lte | Op::Gt | Op::Gte => 8,
			Op::Eq | Op::Neq => 7,
			Op::BitAnd => 6,
			Op::BitXor => 5,
			Op::BitOr => 4,
			Op::And => 3,
			Op::Or => 2,
			Op::Assign | Op::AddEq | Op::SubEq | Op::MulEq | Op::DivEq | Op::RemEq | Op::BitAndEq | Op::BitOrEq | Op::BitXorEq | Op::LShiftEq | Op::RShiftEq => 1,
			_ => 0
		}
	}

	#[allow(clippy::range_plus_one)]
	pub(self) const fn eof_range(&self) -> Range<usize> {
		let len = self.source.len();
		len..len+1
	}

	pub(self) fn is_eof(&mut self) -> bool {
		self.peek().is_none()
	}

	pub(self) fn push_error(&mut self, error: ParseError) {
		let range = if error == ParseError::UnexpectedEOF {
			self.eof_range()
		} else {
			self.range.clone()
		};
		self.errors.push((error, range));
	}

	//

	pub fn parse(&mut self) -> (Block, &Vec<(ParseError, Range<usize>)>) {
		(self.parse_block(), &self.errors)
	}
}

