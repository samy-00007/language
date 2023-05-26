use super::ast::{ParseError, Stmt};
use super::{PResult, Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_let(&mut self) -> PResult<Stmt> {
		let name = self.get_ident()?;

		let mut t = None;
		if self.at(Token::Colon) {
			self.next();
			assert_eq!(self.next(), Some(Token::Identifier));
			t = Some(self.text());
		}

		self.consume(Token::Assign)?; // TODO: variable without initial value
		let expr = self.parse_expression(0)?;
		self.consume(Token::SemiColon)?;
		Ok(Stmt::Local {
			name,
			t,
			val: Box::new(expr)
		})
	}

	fn parse_fn_stmt(&mut self) -> PResult<Stmt> {
		let name = self.get_ident()?;
		self.consume(Token::LParen)?;

		let args = self.parse_fn_args(Token::RParen)?;
		self.next(); // Token::RParen

		self.consume(Token::Colon)?;
		let t = self.get_ident()?;

		self.consume(Token::LBrace)?;
		let block = self.parse_block()?;
		self.consume(Token::RBrace)?;

		Ok(Stmt::Function {
			name,
			args,
			t,
			block
		})
	}

	fn parse_expr(&mut self) -> PResult<Stmt> {
		let expr = self.parse_expression(0)?;
		if !self.at(Token::SemiColon) {
			if !self.at(Token::RBrace) {
				Err(ParseError::ExpectedTokenButNotFound(Token::RBrace))
			} else {
				Ok(Stmt::Return(expr))
			}
		} else {
			self.consume(Token::SemiColon)?;
			Ok(Stmt::Expr(expr))
		}
	}

	fn parse_if(&mut self) -> PResult<Stmt> {
		self.consume(Token::LParen)?;
		let cond = self.parse_expression(0)?;
		self.consume(Token::RParen)?;

		self.consume(Token::LBrace)?;
		let block = self.parse_block()?;
		self.consume(Token::RBrace)?;

		Ok(Stmt::If { cond, block })
	}

	pub fn parse_statement(&mut self) -> PResult<Stmt> {
		let peek = self.peek();
		assert!(peek.is_some(), "Unexpected EOF");
		let peek = peek.unwrap();

		if !self.is_keyword(peek) {
			self.parse_expr()
		} else {
			self.next();
			match peek {
				Token::Let => self.parse_let(),
				Token::Fn => self.parse_fn_stmt(),
				Token::If => self.parse_if(),
				// Token::While | Token::For => panic!("Unhandled keyword"),
				x => todo!("token '{:?}' unhandled (statement)", x)
			}
		}
	}
}
