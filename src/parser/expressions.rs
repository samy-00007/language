use super::ast::{Block, Expr, Literal, Stmt, ParseError};
use super::{Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_ident(&mut self) -> Result<Expr, ParseError> {
		if self.at(Token::LParen) {
			let name = self.text();
			self.next();
			let mut args = Vec::new();

			while !self.at(Token::RParen) {
				args.push(self.parse_expression()?);
				if self.at(Token::Comma) {
					self.next();
				}
			}
			self.next();
			Ok(Expr::FnCall { name, args })
		} else {
			Ok(Expr::Ident(self.text()))
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

	pub(super) fn parse_block(&mut self) -> Result<Block, ParseError> {
		let mut stmts = Vec::new();
		while !matches!(self.peek(), Some(Token::RBrace) | None) {
			let expr = self.parse_statement()?;

			if matches!(expr, Stmt::Return(_)) {
				stmts.push(expr);
				break
			}
			stmts.push(expr);
		}
		Ok(stmts)
	}

	pub fn parse_expression(&mut self) -> Result<Expr, ParseError> {
		let next = self.next().ok_or(ParseError::UnexpectedEOF)?;
		
		let lhs = {
			if self.is_ident(next) {
				self.parse_ident()?
			} else if self.is_lit(next) {
				self.parse_lit(next)
			} else if next == Token::LParen {
				let expr = self.parse_expression()?;
				self.consume(Token::RParen)?;
				expr
			} else {
				return Err(ParseError::UnexpectedToken(next));
			}
		};

		if self.is_next_op() {
			let op = self.next().unwrap().into();
			let rhs = self.parse_expression()?;
			Ok(Expr::Infix {
				// FIXME: priorities
				op,
				lhs: Box::new(lhs),
				rhs: Box::new(rhs)
			})
		} else {
			Ok(lhs)
		}
	}
}


#[cfg(test)]
mod tests {
    use crate::parser::{Parser, ast::{Expr, Literal, Operator, ParseError}};

	#[test]
	fn parse_float() {
		let mut parser = Parser::new("3.5 4.7 7.2");
		while let Ok(x) = parser.parse_expression() {
			// can't compare them precisely because they're floats
			assert!(matches!(x, Expr::Lit(Literal::Float(_)))); 
		}
	}

	#[test]
	fn parse_lit() {
		let mut parser = Parser::new("5 \"abcd\" true false");
		let expected = vec![
			Expr::Lit(Literal::Int(5)),
			Expr::Lit(Literal::String(String::from("abcd"))),
			Expr::Lit(Literal::Bool(true)),
			Expr::Lit(Literal::Bool(false))
		];
		let mut parsed = Vec::new();
		while let Ok(x) = parser.parse_expression() {
			parsed.push(x);
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	fn parse_ident() {
		let mut parser = Parser::new("abcd print(5) test");
		let expected = vec![
			Expr::Ident("abcd".to_string()),
			Expr::FnCall {
				name: "print".to_string(),
				args: vec![Expr::Lit(Literal::Int(5))]
			},
			Expr::Ident("test".to_string()),
		];
		let mut parsed = Vec::new();
		while let Ok(x) = parser.parse_expression() {
			parsed.push(x);
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	#[allow(clippy::just_underscores_and_digits)]
	fn parse_ops() {
		let mut parser = Parser::new("5+5 6*7 5^3 4-8 4/8 8!=4 5==5 6>3 4>=4 1<5 6<=test 4*4*4");
		let f = |n| Box::new(Expr::Lit(Literal::Int(n)));

		let _1 = f(1);
		let _3 = f(3);
		let _4 = f(4);
		let _5 = f(5);
		let _6 = f(6);
		let _7 = f(7);
		let _8 = f(8);

		let expected = vec![
			Expr::Infix { op: Operator::Plus, lhs: _5.clone(), rhs: _5.clone() },
			Expr::Infix { op: Operator::Mul, lhs: _6.clone(), rhs: _7 },
			Expr::Infix { op: Operator::Xor, lhs: _5.clone(), rhs: _3.clone() },
			Expr::Infix { op: Operator::Sub, lhs: _4.clone(), rhs: _8.clone() },
			Expr::Infix { op: Operator::Div, lhs: _4.clone(), rhs: _8.clone() },
			Expr::Infix { op: Operator::Neq, lhs: _8, rhs: _4.clone() },
			Expr::Infix { op: Operator::Eq, lhs: _5.clone(), rhs: _5.clone() },
			Expr::Infix { op: Operator::Gt, lhs: _6.clone(), rhs: _3 },
			Expr::Infix { op: Operator::Gte, lhs: _4.clone(), rhs: _4.clone() },
			Expr::Infix { op: Operator::Lt, lhs: _1, rhs: _5 },
			Expr::Infix { op: Operator::Lte, lhs: _6, rhs: Box::new(Expr::Ident("test".to_string())) },
			Expr::Infix { op: Operator::Mul, lhs: Box::new(Expr::Infix { op: Operator::Mul, lhs: _4.clone(), rhs: _4.clone() }), rhs: _4 }
		];
		let mut parsed = Vec::new();
		loop {
			let x = parser.parse_expression();
			match x {
				Ok(x) => parsed.push(x),
				Err(ParseError::UnexpectedEOF) => break,
				Err(x) => {
					eprintln!("{}", x);
					eprintln!("{:?}", parser.range);
					panic!()
				}
			
			}
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	fn parse_priority() {
		let mut parser = Parser::new("6*7*5  3*5 + 5*5  7*7*7+3 6/7*8-2  a & b & c  a && b && c");
		let expected = vec![
			
		];
		let mut parsed = Vec::new();
		while let Ok(x) = parser.parse_expression() {
			parsed.push(x);
		}

		assert_eq!(parsed, expected);
	}
}