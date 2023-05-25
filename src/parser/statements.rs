use core::panic;

use super::ast::Stmt;
use super::{Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_let(&mut self) -> Stmt {
		let name = self.get_ident();

		let mut t = None;
		if self.at(Token::Colon) {
			self.next();
			assert_eq!(self.next(), Some(Token::Identifier));
			t = Some(self.text());
		}

		self.consume(Token::Assign); // TODO: variable without initial value
		let expr = self.parse_expression();
		self.consume(Token::SemiColon);
		Stmt::Local {
			name,
			t,
			val: Box::new(expr)
		}
	}

	fn parse_fn(&mut self) -> Stmt {
		let name = self.get_ident();
		self.consume(Token::LParen);

		let mut args = Vec::new();
		while !self.at(Token::RParen) {
			let arg_name = self.get_ident();
			self.consume(Token::Colon);
			let t = self.get_ident();
			args.push((arg_name, t));

			if self.at(Token::Comma) {
				self.next();
				if self.at(Token::RParen) {
					panic!("Unexpected trailing comma") // TODO: change that
				}
			}
		}

		self.consume(Token::Colon);
		let t = self.get_ident();

		self.consume(Token::LBrace);
		let block = self.parse_block();
		self.consume(Token::RBrace);

		Stmt::Function {
			name,
			args,
			t,
			block
		}
	}

	fn parse_expr(&mut self) -> Stmt {
		let expr = self.parse_expression();
		if !self.at(Token::SemiColon) {
			if !self.at(Token::RBrace) {
				panic!("Expected semicolon")
			} else {
				Stmt::Return(expr)
			}
		} else {
			self.consume(Token::SemiColon);
			Stmt::Expr(expr)
		}
	}

	fn parse_if(&mut self) -> Stmt {
		self.consume(Token::LParen);
		let cond = self.parse_expression();
		self.consume(Token::RParen);

		self.consume(Token::LBrace);
		let block = self.parse_block();
		self.consume(Token::RBrace);

		Stmt::If { cond, block }
	}

	pub fn parse_statement(&mut self) -> Stmt {
		let peek = self.peek();
		assert!(peek.is_some(), "Unexpected EOF");
		let peek = peek.unwrap();

		if !self.is_keyword(peek) {
			self.parse_expr()
		} else {
			self.next();
			match peek {
				Token::Let => self.parse_let(),
				Token::Fn => self.parse_fn(),
				Token::If => self.parse_if(),
				// Token::While | Token::For => panic!("Unhandled keyword"),
				x => todo!("token '{:?}' unhandled (statement)", x)
			}
		}
	}
}
