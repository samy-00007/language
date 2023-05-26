pub mod ast;
mod expressions;
mod statements;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::ast::{Block, ParseError, Operator};

pub(super) type RetItem = (Result<Token, ()>, Range<usize>);
pub(super) type Item = (Token, Range<usize>);
pub(super) type PResult<T> = Result<T, ParseError>;

pub struct Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	tokens: Peekable<I>,
	source: &'a str,
	range: Range<usize> // phantom: PhantomData<&'a I>
}

impl<'a> Parser<'a, SpannedIter<'a, Token>> {
	pub fn new(source: &'a str) -> Parser<'a, SpannedIter<'a, Token>> {
		let lex = Token::lexer(source);
		Self {
			tokens: lex.spanned().peekable(),
			source,
			range: 0..0 // phantom: PhantomData
		}
	}
}

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn unwrap_ref(t: Option<&RetItem>) -> Option<Item> {
		match t {
			Some(&(Ok(ref token), ref r)) => Some((token.to_owned(), r.to_owned())),
			_ => None
		}
	}

	fn unwrap(t: Option<RetItem>) -> Option<Item> {
		match t {
			Some((Ok(token), r)) => Some((token, r)),
			_ => None
		}
	}

	pub(self) fn peek(&mut self) -> Option<Token> {
		let next = Parser::<'a, I>::unwrap_ref(self.tokens.peek());
		if let Some((t, _)) = next {
			Some(t)
		} else {
			None
		}
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
		if let Some(token) = self.peek() {
			token == expected
		} else {
			false
		}
	}

	pub(self) fn text(&self) -> String {
		self.source[self.range.to_owned()].to_string()
	}

	pub(self) fn consume(&mut self, expected: Token) -> PResult<()> {
		match self.next() {
			Some(token) => {
				if token == expected {
					Ok(())
				} else {
					Err(ParseError::ExpectedTokenButFoundInstead(expected, token))
				}
			}
			None => Err(ParseError::UnexpectedEOF)
		}
	}

	// utils

	pub(self) fn is_lit(&self, token: Token) -> bool {
		matches!(
			token,
			Token::String | Token::Int | Token::Float | Token::True | Token::False
		)
	}

	pub(self) fn is_ident(&self, token: Token) -> bool {
		token == Token::Identifier
	}

	pub(self) fn is_op(&self, token: Token) -> bool {
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
		)
	}

	pub(self) fn is_keyword(&self, token: Token) -> bool {
		matches!(
			token,
			Token::Fn | Token::Let | Token::If | Token::For | Token::While
		)
	}

	pub(self) fn get_ident(&mut self) -> PResult<String> {
		let ident = self.next().ok_or(ParseError::UnexpectedEOF)?;
		if ident != Token::Identifier {
			Err(ParseError::ExpectedTokenButFoundInstead(
				Token::Identifier,
				ident
			))
		} else {
			Ok(self.text())
		}
	}

	pub(self) fn operator_precedence(&self, op: Operator) -> usize {
		// https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages
		type Op = Operator;
		match op {
			Op::Not | Op::BitNot => 12,
			Op::Exponent => 11,
			Op::Mul | Op::Div | Op::Rem => 10,
			Op::Add | Op::Sub  => 9,
			Op::LShift | Op::RShift => 8,
			Op::Lt | Op::Lte | Op::Gt | Op::Gte => 7,
			Op::Eq | Op::Neq => 6,
			Op::BitAnd => 5,
			Op::BitXor => 4,
			Op::BitOr => 3,
			Op::And => 2,
			Op::Or => 1,
			_ => 0
		}
	}

	//

	pub fn parse(&mut self) -> PResult<Block> {
		self.parse_block()
	}
}

// FIXME: remove panics for proper error handling
