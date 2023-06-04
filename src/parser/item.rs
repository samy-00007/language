//! Item module
//! Items are what are at the top level of a module, either in a file or a `mod name { ... }`
use crate::lexer::Token;

use super::{
	ast::{Item, ParseError, Ty},
	Parser, RetItem
};

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = RetItem>
{
	fn parse_fn(&mut self) -> Item {
		let name = self.get_ident();

		self.consume(Token::LParen);
		let args = self.parse_fn_args(Token::RParen);
		self.next(); // Token::RParen

		let t = if self.at(Token::Arrow) {
			self.consume(Token::Arrow);
			self.parse_ty()
		} else {
			Ty::None
		};

		self.consume(Token::LBrace);
		let block = self.parse_block();
		self.consume(Token::RBrace);

		Item::Function {
			name,
			args,
			t,
			block
		}
	}

	fn parse_struct(&mut self) -> Item {
		let name = self.get_ident();
		self.consume(Token::LBrace);

		let fields = self.parse_fn_args(Token::RBrace); // TODO: support visibility (pub)
		self.next();
		Item::Struct { name, fields }
	}

	fn parse_const(&mut self) -> Item {
		let name = self.get_ident();

		self.consume(Token::Colon);
		let ty = self.parse_ty();

		self.consume(Token::Eq);
		let value = self.parse_expression(0);
		self.consume(Token::SemiColon);

		Item::Constant { name, ty, value }
	}

	pub(super) fn parse_item(&mut self) -> Option<Item> {
		match self.next() {
			Some(Token::Fn) => Some(self.parse_fn()),
			Some(Token::Struct) => Some(self.parse_struct()),
			Some(Token::Const) => Some(self.parse_const()),
			Some(x) => {
				self.push_error(ParseError::UnexpectedToken(x));
				None
			}
			None => None
		}
	}
}

#[cfg(test)]
mod tests {}
