pub mod ast;
mod expressions;
mod statements;

use crate::lexer::Token;
use logos::{Logos, SpannedIter};
use std::{iter::Peekable, ops::Range};

use self::ast::Block;

pub(super) type RetItem = (Result<Token, ()>, Range<usize>);
pub(super) type Item = (Token, Range<usize>);

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

	pub(self) fn consume(&mut self, expected: Token) {
		let token = self.next();

		assert!(
			token.is_some(),
			"Expected {:?} but found None (EOF)",
			expected
		);

		let token = token.unwrap();

		assert_eq!(
			token, expected,
			"Expected {:?} but found {:?}",
			expected, token
		);
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
			Token::Plus | Token::Minus | Token::Asterisk | Token::Slash | Token::Eq
		)
	}

	pub(self) fn is_next_op(&mut self) -> bool {
		let peek = self.peek();
		if let Some(token) = peek {
			self.is_op(token)
		} else {
			false
		}
	}

	pub(self) fn get_ident(&mut self) -> String {
		let ident = self.next().expect("Expected identifier, found EOF");
		assert_eq!(ident, Token::Identifier);
		self.text()
	}

	//

	pub fn parse(&mut self) -> Block {
		self.parse_block()
	}
}

// FIXME: remove panics for proper error handling
