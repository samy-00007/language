use super::ast::{Block, Expr, Literal, ParseError, Stmt, Operator, Prefix};
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
			let args = self.parse_list(false, Token::RParen);
			self.consume(Token::RParen);
			Expr::FnNamedCall { name, args }
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
						.unwrap_or_else(|_| {
							self.push_error(ParseError::IntParseError(text)); // FIXME: parse nums with e (e.g 10e2)
							0
						})
				))
			}
			Token::Float => {
				let text = self.text();
				Expr::Lit(Literal::Float(
					//f128::f128::parse(&text)
					text.parse()
						.unwrap_or_else(|_| {
							self.push_error(ParseError::FloatParseError(text));
							0.
						})
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
			let stmt = self.parse_statement();

			if matches!(stmt, Stmt::Return(_)) {
				stmts.push(stmt);
				break;
			}
			stmts.push(stmt);
		}
		stmts
	}

	/// Parse a list of expression / arguments
	/// It does not consume `end_token`
	///
	/// Example of list that it parses:
	/// ```
	/// // self.parse_list(true, Token::RParen)
	/// ident1: type1, ident2: type2, ident3: type3
	///
	/// ```
	// FIXME: parse something else than ident, like `object.property`
	pub(super) fn parse_fn_args(&mut self, end_token: Token) -> Vec<(String, String)> {
		let mut args = Vec::new();
		loop {
			let peek = self.peek();
			match peek {
				None => {
					self.push_error(ParseError::UnexpectedEOF);
					break;
				},
				Some(x) => {
					if x == end_token || x == Token::SemiColon {
						break
					}
				}
			}
	
			let arg = self.get_ident();
			self.consume(Token::Colon);
			let t = self.get_ident();
			args.push((arg, t));
			if !self.at(end_token) {
				self.consume(Token::Comma);
			} else if self.at(Token::Comma) { // trailing comma
				self.next();
			}
			if self.is_eof() {
				self.push_error(ParseError::UnexpectedEOF);
				break;
			}
		}
		args
	}

	pub(super) fn parse_list(&mut self, only_idents: bool, end_token: Token) -> Vec<Expr> {
		let mut args = Vec::new();
		loop {
			let peek = self.peek();
			match peek {
				None => {
					self.push_error(ParseError::UnexpectedEOF);
					break;
				},
				Some(x) => {
					if x == end_token || x == Token::SemiColon {
						break
					}
				}
			}

			let arg = self.parse_expression(0);
			if only_idents && !matches!(arg, Expr::Ident(_)) {
				self.push_error(ParseError::ExpectedExprButFoundInstead(
					Expr::Ident(String::new()),
					arg
				));
				args.push(Expr::Error);
			} else {
				args.push(arg);
			}
			if !self.at(end_token) {
				self.consume(Token::Comma);
			} else if self.at(Token::Comma) { // trailing comma
				self.next();
			}
			if self.is_eof() {
				self.push_error(ParseError::UnexpectedEOF);
				break;
			}
		}
		args
	}

	pub fn parse_fn_call(&mut self, lhs: Expr) -> Expr {
		self.next(); // known to be Token::LParen
		let args = self.parse_list(true, Token::RParen);
		self.next();
		Expr::FnCall {
			expr: Box::new(lhs),
			args
		}
	}

	pub fn parse_expression(&mut self, precedence: usize) -> Expr {
		let Some(next) = self.next() else {
			self.push_error(ParseError::UnexpectedEOF);
			return Expr::Error
		};

		let mut lhs = {
			if Self::is_ident(next) {
				self.parse_ident()
			} else if Self::is_lit(next) {
				self.parse_lit(next)
			} else if next == Token::LParen {
				let expr = self.parse_expression(0);
				self.consume(Token::RParen);
				expr
			} else if next == Token::LBrace {
				let blk = self.parse_block();
				self.consume(Token::RBrace);
				Expr::Block(blk)
			} else if Self::is_op(next) {
				let expr = self.parse_expression(0);
				let op: Operator = next.into();
				let op = op.try_into().unwrap_or_else(|e| {self.push_error(e); Prefix::Err});
				Expr::Prefix(op, Box::new(expr))
			} else {
				self.push_error(ParseError::UnexpectedToken(next));
				Expr::Error
			}
		};

		loop {
			if let Some(peek) = self.peek() {
				if Self::is_op(peek) {
					let op = peek.into();
					let r_precedence = Self::operator_precedence(op);
					if precedence >= r_precedence {
						return lhs;
					}
					self.next();
					let rhs = self.parse_expression(r_precedence);
					lhs = Expr::Infix {
						// FIXME: priorities
						op,
						lhs: Box::new(lhs),
						rhs: Box::new(rhs)
					};
				} else if peek == Token::LParen {
					lhs = self.parse_fn_call(lhs);
				} else {
					return lhs;
				}
			} else {
				return lhs;
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::{
		lexer::Token,
		parser::{
			ast::{Expr, Literal, Operator, Stmt},
			Parser
		}
	};

	#[test]
	fn parse_args() {
		let mut parser = Parser::new("abcd, efgh, uch65)");
		let args = parser.parse_list(true, Token::RParen);
		assert_eq!(
			args,
			vec![
				Expr::Ident("abcd".to_string()),
				Expr::Ident("efgh".to_string()),
				Expr::Ident("uch65".to_string())
			]
		);

		let mut parser = Parser::new("5, {print(\"test\"); 5}, abcd)");
		let args = parser.parse_list(false, Token::RParen);
		assert_eq!(
			args,
			vec![
				Expr::Lit(Literal::Int(5)),
				Expr::Block(vec![
					Stmt::Expr(Expr::FnNamedCall {
						name: "print".to_string(),
						args: vec![Expr::Lit(Literal::String("test".to_string()))]
					}),
					Stmt::Return(Expr::Lit(Literal::Int(5)))
				]),
				Expr::Ident("abcd".to_string())
			]
		);

		let mut parser = Parser::new("abcd: number, efgh: bool, uch65: string)");
		let args = parser.parse_fn_args(Token::RParen);
		assert_eq!(
			args,
			vec![
				("abcd".to_string(), "number".to_string()),
				("efgh".to_string(), "bool".to_string()),
				("uch65".to_string(), "string".to_string())
			]
		);
	}

	#[test]
	fn parse_float() {
		let mut parser = Parser::new("3.5 4.7 7.2");
		loop {
			let x = parser.parse_expression(0);
			if x == Expr::Error { // FIXME:
				break
			}
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
			Expr::Lit(Literal::Bool(false)),
		];
		let mut parsed = Vec::new();
		loop {
			let x = parser.parse_expression(0);
			if x == Expr::Error {
				break
			}
			parsed.push(x);
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	fn parse_ident() {
		let mut parser = Parser::new("abcd print(5) test");
		let expected = vec![
			Expr::Ident("abcd".to_string()),
			Expr::FnNamedCall {
				name: "print".to_string(),
				args: vec![Expr::Lit(Literal::Int(5))]
			},
			Expr::Ident("test".to_string()),
		];
		let mut parsed = Vec::new();
		loop {
			let x = parser.parse_expression(0);
			if x == Expr::Error {
				break
			}
			parsed.push(x);
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	fn parse_block() {
		let strs = [
			(
				"5; print(\"test\");4}",
				vec![
					Stmt::Expr(Expr::Lit(Literal::Int(5))),
					Stmt::Expr(Expr::FnNamedCall {
						name: "print".to_string(),
						args: vec![Expr::Lit(Literal::String("test".to_string()))]
					}),
					Stmt::Return(Expr::Lit(Literal::Int(4))),
				]
			),
			(
				"5; print(\"test\");4;", // returning a value requires to be in a block surrounded with braces
				vec![
					Stmt::Expr(Expr::Lit(Literal::Int(5))),
					Stmt::Expr(Expr::FnNamedCall {
						name: "print".to_string(),
						args: vec![Expr::Lit(Literal::String("test".to_string()))]
					}),
					Stmt::Expr(Expr::Lit(Literal::Int(4))),
				]
			)
		];
		for (string, expected) in strs {
			let mut parser = Parser::new(string);
			let res = parser.parse_block();

			assert_eq!(res, expected);
		}
	}

	#[test]
	#[allow(clippy::just_underscores_and_digits)]
	fn parse_ops() {
		let mut parser = Parser::new("5+5 6*7 5^3 4-8 4/8 8!=4 5==5 6>3 4>=4 1<5 6<=test 4*4*4");
		let f = |n| Box::new(Expr::Lit(Literal::Int(n)));

		let n1 = f(1);
		let n3 = f(3);
		let n4 = f(4);
		let n5 = f(5);
		let n6 = f(6);
		let n7 = f(7);
		let n8 = f(8);

		let expected = vec![
			Expr::Infix {
				op: Operator::Add,
				lhs: n5.clone(),
				rhs: n5.clone()
			},
			Expr::Infix {
				op: Operator::Mul,
				lhs: n6.clone(),
				rhs: n7
			},
			Expr::Infix {
				op: Operator::BitXor,
				lhs: n5.clone(),
				rhs: n3.clone()
			},
			Expr::Infix {
				op: Operator::Sub,
				lhs: n4.clone(),
				rhs: n8.clone()
			},
			Expr::Infix {
				op: Operator::Div,
				lhs: n4.clone(),
				rhs: n8.clone()
			},
			Expr::Infix {
				op: Operator::Neq,
				lhs: n8,
				rhs: n4.clone()
			},
			Expr::Infix {
				op: Operator::Eq,
				lhs: n5.clone(),
				rhs: n5.clone()
			},
			Expr::Infix {
				op: Operator::Gt,
				lhs: n6.clone(),
				rhs: n3
			},
			Expr::Infix {
				op: Operator::Gte,
				lhs: n4.clone(),
				rhs: n4.clone()
			},
			Expr::Infix {
				op: Operator::Lt,
				lhs: n1,
				rhs: n5
			},
			Expr::Infix {
				op: Operator::Lte,
				lhs: n6,
				rhs: Box::new(Expr::Ident("test".to_string()))
			},
			Expr::Infix {
				op: Operator::Mul,
				lhs: Box::new(Expr::Infix {
					op: Operator::Mul,
					lhs: n4.clone(),
					rhs: n4.clone()
				}),
				rhs: n4
			},
		];
		let mut parsed = Vec::new();
		loop {
			let x = parser.parse_expression(0);
			match x {
				Expr::Error => break, // FIXME:
				_ => parsed.push(x)
			}
		}

		assert_eq!(parsed, expected);
	}

	#[test]
	fn parse_priority() {
		let mut parser = Parser::new("6*7*5  3*5 + 5*5  7*7*7+3 6/7-2*8-2  a & b & c  a && b && c");
		let expected = vec![
			"((6 * 7) * 5)",
			"((3 * 5) + (5 * 5))",
			"(((7 * 7) * 7) + 3)",
			"(((6 / 7) - (2 * 8)) - 2)",
			"((a & b) & c)",
			"((a && b) && c)"
		];
		let mut parsed = Vec::new();
		loop {
			let x = parser.parse_expression(0);
			if x == Expr::Error {
				break
			}
			parsed.push(x.to_string());
		}

		assert_eq!(parsed, expected);
	}
}
