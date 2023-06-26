use language_ast::{Expr, Generic, Stmt};
use crate::error::ParseError;
use super::{Parser, RetItem};
use crate::lexer::Token;

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_let(&mut self) -> Stmt {
		let name = self.get_ident();

		let ty = if self.at(Token::Colon) {
			self.next();
			Some(self.parse_ty())
		} else {
			None
		};

		if ty.is_none() && !self.allow_implicit_types {
			self.push_error(ParseError::NoImplicitTypeAllowed);
		}

		self.consume(Token::Eq); // TODO: variable without initial value
		let expr = self.parse_expression(0);
		self.consume(Token::SemiColon);
		Stmt::Local {
			name,
			ty,
			val: Box::new(expr)
		}
	}

	fn _parse_generics(&mut self) -> Vec<Generic> {
		self.parse_l(Token::RChevron, |this| {
			let name = this.get_ident();
			let mut traits = Vec::new();
			if this.at(Token::Colon) {
				this.next();

				traits.push(this.get_ident());
				while this.at(Token::Plus) {
					this.next();
					traits.push(this.get_ident());
				}
			}
			Generic { name, traits }
		})
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
		let Some(peek) = self.peek() else {
			self.push_error(ParseError::UnexpectedEOF);
			return Stmt::Error
		};

		if Self::is_item_start(peek) {
			Stmt::Item(unsafe { self.parse_item().unwrap_unchecked() })
		} else if Self::is_keyword(peek) {
			self.next();
			match peek {
				Token::Let => self.parse_let(),
				// Token::Fn => self.parse_fn_stmt(),
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

#[cfg(test)]
mod tests {
	use language_ast::{Expr, Literal, Operator, Stmt, Ty};
	use crate::parser::Parser;
	use pretty_assertions::assert_eq;

	#[test]
	fn parse_let() {
		let mut parser = Parser::new("let abcd: number = 10;");
		let expected = vec![Stmt::Local {
			name: "abcd".into(),
			ty: Some(Ty::Ident("number".into())),
			val: Expr::Lit(Literal::Int(10)).into()
		}];

		let parsed = parser.parse();

		assert_eq!(parsed.0, expected);
		assert_eq!(parsed.1.len(), 0);
	}
	/*
		#[test]
		fn parse_fn() {
			let mut parser = Parser::new(
				"
			fn abcd() -> number {
				5
			}

			fn efgh<T: Display>(a: T) {
				print(a)
			}

			fn square(b: number) -> number {
				b * b
			}
			"
			);

			let expected = vec![
				Stmt::Function {
					name: "abcd".into(),
					generics: vec![],
					args: vec![],
					t: Some("number".into()),
					block: vec![Stmt::Return(Expr::Lit(Literal::Int(5)))]
				},
				Stmt::Function {
					name: "efgh".into(),
					generics: vec![Generic {
						name: "T".into(),
						traits: vec!["Display".into()]
					}],
					args: vec![Argument {
						name: "a".into(),
						ty: "T".into()
					}],
					t: None,
					block: vec![Stmt::Return(Expr::FnNamedCall {
						name: "print".into(),
						args: vec![Expr::Ident("a".into())]
					})]
				},
				Stmt::Function {
					name: "square".into(),
					generics: vec![],
					args: vec![Argument {
						name: "b".into(),
						ty: "number".into()
					}],
					t: Some("number".into()),
					block: vec![Stmt::Return(Expr::Infix {
						op: Operator::Mul,
						lhs: Expr::Ident("b".into()).into(),
						rhs: Expr::Ident("b".into()).into()
					})]
				},
			];

			let parsed = parser.parse();

			assert_eq!(parsed.0, expected);
			assert_eq!(parsed.1.len(), 0);
		}
	*/
	#[test]
	fn parse_return() {
		let mut parser = Parser::new("return abcd;");

		let expected = vec![Stmt::FnReturn(Expr::Ident("abcd".into()))];

		let parsed = parser.parse();

		assert_eq!(parsed.0, expected);
		assert_eq!(parsed.1.len(), 0);
	}

	// no test for while because it is litteraly the same code as for "if"
	#[test]
	fn parse_if() {
		let mut parser = Parser::new("if (a > 1) { print(a); }");

		let expected = vec![Stmt::If {
			cond: Expr::Infix {
				op: Operator::Gt,
				lhs: Expr::Ident("a".into()).into(),
				rhs: Expr::Lit(Literal::Int(1)).into()
			},
			block: vec![Stmt::Expr(Expr::FnNamedCall {
				name: "print".into(),
				args: vec![Expr::Ident("a".into())]
			})]
		}];

		let parsed = parser.parse();

		assert_eq!(parsed.0, expected);
		assert_eq!(parsed.1.len(), 0);
	}

	#[test]
	fn parse_cond_block() {
		let mut parser = Parser::new("(a <= 1) { print(a); }");

		let cond_expected = Expr::Infix {
			op: Operator::Lte,
			lhs: Expr::Ident("a".into()).into(),
			rhs: Expr::Lit(Literal::Int(1)).into()
		};
		let block_expected = vec![Stmt::Expr(Expr::FnNamedCall {
			name: "print".into(),
			args: vec![Expr::Ident("a".into())]
		})];

		let parsed = parser.parse_cond_block();

		assert_eq!(parsed.0, cond_expected);
		assert_eq!(parsed.1, block_expected);
		assert_eq!(parser.errors().len(), 0);
	}
}
