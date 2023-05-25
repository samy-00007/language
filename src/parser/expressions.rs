use super::ast::{Block, Expr, Literal, Stmt};
use super::{Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_ident(&mut self) -> Expr {
		if self.at(Token::LParen) {
			let name = self.text();
			self.next();
			let mut args = Vec::new();

			while !self.at(Token::RParen) {
				args.push(self.parse_expression());
				if self.at(Token::Comma) {
					self.next();
					if self.at(Token::RParen) {
						panic!("Unexpected trailing comma") // TODO: change that
					}
				}
			}
			self.next();
			Expr::FnCall { name, args }
		} else {
			Expr::Ident(self.text())
		}
	}

	fn parse_lit(&mut self, token: Token) -> Expr {
		match token {
			Token::Int => {
				let text = self.text();
				Expr::Lit(Literal::Int(
					text.parse()
						.unwrap_or_else(|_| panic!("invalid integer: {}", text))
				))
			}
			Token::Float => {
				let text = self.text();
				Expr::Lit(Literal::Float(
					f128::f128::parse(&text)
						.unwrap_or_else(|_| panic!("invalid integer: {}", text))
				))
			}
			Token::True => Expr::Lit(Literal::Bool(true)),
			Token::False => Expr::Lit(Literal::Bool(false)),
			Token::String => {
				let r = (self.range.start + 1)..(self.range.end - 1);
				let text = self.source[r].to_string();
				Expr::Lit(Literal::String(text))
			}
			_ => unreachable!()
		}
	}

	pub(super) fn parse_block(&mut self) -> Block {
		let mut stmts = Vec::new();
		while !matches!(self.peek(), Some(Token::RBrace) | None) {
			let expr = self.parse_statement();

			if matches!(expr, Stmt::Return(_)) {
				stmts.push(expr);
				break
			}
			stmts.push(expr);
		}
		stmts
	}

	pub fn parse_expression(&mut self) -> Expr {
		let next = self.next();
		assert!(next.is_some(), "Unexpected EOF");
		let next = next.unwrap();
		let lhs = {
			if self.is_ident(next) {
				self.parse_ident()
			} else if self.is_lit(next) {
				self.parse_lit(next)
			} else if next == Token::LParen {
				let expr = self.parse_expression();
				self.consume(Token::RParen);
				expr
			} else {
				todo!("token '{:?}' unhandled (expression)", next);
			}
		};

		if self.is_next_op() {
			let op = self.next().unwrap().into();
			let rhs = self.parse_expression();
			Expr::Infix {
				// FIXME: priorities
				op,
				lhs: Box::new(lhs),
				rhs: Box::new(rhs)
			}
		} else {
			lhs
		}
	}
}


#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ast::{Expr, Literal}};

	#[test]
	fn parse_lit() {
		let mut parser = Parser::new("5 3.5 \"abcd\" true false");
		let expected = vec![
			Expr::Lit(Literal::Int(5)),
			Expr::Lit(Literal::Float((3.5).into())),
			Expr::Lit(Literal::String(String::from("abcd"))),
			Expr::Lit(Literal::Bool(true)),
			Expr::Lit(Literal::Bool(false))
		];
		let mut parsed = Vec::new();
		for _ in 0..5 {
			let e = parser.parse_expression();
			parsed.push(e);
		}

		assert_eq!(parsed, expected);
	}
}