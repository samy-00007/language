pub mod ast;
mod expressions;
mod statements;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::ast::{Block, ParseError, Operator};

pub type RetItem = (Result<Token, ()>, Range<usize>);
pub type Item = (Token, Range<usize>);
pub type PResult<T> = Result<T, ParseError>;

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
		self.peek().map_or(false, |token| token == expected)
	}

	pub(self) fn text(&self) -> String {
		self.source[self.range.clone()].to_string()
	}

	pub(self) fn consume(&mut self, expected: Token) -> PResult<()> {
		self.next().map_or_else(|| Err(ParseError::UnexpectedEOF), |token| if token == expected {
					Ok(())
				} else {
					Err(ParseError::ExpectedTokenButFoundInstead(expected, token))
				})
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
			Token::Fn | Token::Let | Token::If | Token::For | Token::While
		)
	}

	pub(self) fn get_ident(&mut self) -> PResult<String> {
		let ident = self.next().ok_or(ParseError::UnexpectedEOF)?;
		if ident == Token::Identifier {
			Ok(self.text())
		} else {
			Err(ParseError::ExpectedTokenButFoundInstead(
				Token::Identifier,
				ident
			))
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

	//

	pub fn parse(&mut self) -> PResult<Block> {
		self.parse_block()
	}
}

