use super::ast::{ParseError, Stmt, Expr};
use super::{Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_let(&mut self) -> Stmt {
		let name = self.get_ident();

		let t = if self.at(Token::Colon) {
			self.next();
			if self.at(Token::Identifier) {
				Some(self.text())
			} else {
				let peek = self.peek().unwrap();
				self.push_error(ParseError::ExpectedTokenButFoundInstead(Token::Identifier, peek));
				None
			}
		} else {
			None
		};

		self.consume_raw(Token::Eq, true); // TODO: variable without initial value
		let expr = self.parse_expression(0);
		self.consume(Token::SemiColon);
		Stmt::Local {
			name,
			t,
			val: Box::new(expr)
		}
	}

	fn parse_fn_stmt(&mut self) -> Stmt {
		let name = self.get_ident();
		self.consume(Token::LParen);

		let args = self.parse_fn_args(Token::RParen);
		self.next(); // Token::RParen

		let t = if self.at(Token::Arrow) {
			self.consume(Token::Arrow);
			Some(self.get_ident())
		} else {
			None
		};

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
		let expr = self.parse_expression(0);
		if self.at(Token::SemiColon) {
			self.consume(Token::SemiColon);
			Stmt::Expr(expr)
		} else if self.at(Token::RBrace) {
			Stmt::Return(expr)
		} else {
			self.push_error(ParseError::ExpectedTokenButNotFound(Token::SemiColon));
			Stmt::Expr(expr)
		}
	}

	fn parse_cond_block(&mut self) -> (Expr, Vec<Stmt>) {
		self.consume(Token::LParen);
		let cond = self.parse_expression(0);
		self.consume(Token::RParen);

		self.consume(Token::LBrace);
		let block = self.parse_block();
		self.consume(Token::RBrace);
		(cond, block)
	}

	fn parse_if(&mut self) -> Stmt {
		let (cond, block) = self.parse_cond_block();
		Stmt::If { cond, block }
		// TODO: else
	}

	fn parse_while(&mut self) -> Stmt {
		let (cond, block) = self.parse_cond_block();
		Stmt::While { cond, block }
	}

	fn parse_return(&mut self) -> Stmt {
		Stmt::FnReturn(self.parse_expression(0))
	}

	pub fn parse_statement(&mut self) -> Stmt {
		while self.at(Token::SemiColon) {
			self.next(); // a bit janky
		}
		let Some(peek) = self.peek() else {
			self.push_error(ParseError::UnexpectedEOF);
			return Stmt::Error
		};

		if Self::is_keyword(peek) {
			self.next();
			match peek {
				Token::Let => self.parse_let(),
				Token::Fn => self.parse_fn_stmt(),
				Token::If => self.parse_if(),
				Token::While => self.parse_while(),
				Token::Return => self.parse_return(),
				x => todo!("token '{:?}' unhandled (statement)", x)
			}
		} else {
			self.parse_expr()
		}
	}
}
