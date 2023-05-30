use logos::{FilterResult, Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
#[logos(skip r"[ \t\n]+")]
pub enum Token {
	#[regex("[A-Za-z_][A-Za-z_0-9]*")] // TODO: maybe support unicode ?
	Identifier,
	#[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
	String,
	#[regex(r"[0-9][0-9_]*(\.[0-9_]+)([eE][\+-]?[0-9_]+)?", priority = 2)] // FIXME: parse nums with e
	Float,
	#[regex(r"[0-9][0-9_]*([eE][\+-]?[0-9_]+)?", priority = 2)]
	// TODO: expand that (0x, 0b, ...)
	Int,

	// keywords
	#[token("let")]
	Let,
	#[token("if")]
	If,
	#[token("while")]
	While,
	#[token("for")]
	For,
	#[token("fn")]
	Fn,
	#[token("return")]
	Return,

	#[token("true")]
	True,
	#[token("false")]
	False,

	// operators
	#[token("+")]
	Plus,
	#[token("+=", priority = 2)]
	PlusEq,
	#[token("-")]
	Minus,
	#[token("-=", priority = 2)]
	MinusEq,
	#[token("*")]
	Asterisk,
	#[token("*=", priority = 2)]
	AsteriskEq,
	#[token("**")]
	DoubleAsterisk,
	#[token("**=", priority = 2)]
	DoubleAsteriskEq,
	#[token("/")]
	Slash,
	#[token("/=", priority = 2)]
	SlashEq,
	#[token("%")]
	Percent,
	#[token("%=", priority = 2)]
	PercentEq,

	// bitwise operations
	#[token("&")]
	Anpersand,
	#[token("&=", priority = 2)]
	AnpersandEq,
	#[token("|")]
	Bar,
	#[token("|=", priority = 2)]
	BarEq,
	#[token("^")]
	Caret,
	#[token("^=", priority = 2)]
	CaretEq,
	#[token("<<", priority = 2)]
	LShift,
	#[token("<<=", priority = 3)]
	LShiftEq,
	#[token(">>", priority = 2)]
	RShift,
	#[token(">>=", priority = 3)]
	RShiftEq,

	// comparaison operator
	#[token(">=", priority = 2)]
	Gte,
	#[token("<=", priority = 2)]
	Lte,
	#[token("==", priority = 2)]
	DoubleEq,
	#[token("!=", priority = 2)]
	Neq,
	#[token("&&", priority = 2)]
	And,
	#[token("&&=", priority = 3)]
	AndEq,
	#[token("||", priority = 2)]
	Or,
	#[token("||=", priority = 3)]
	OrEq,

	#[token("++", priority = 2)]
	Increment,
	#[token("--", priority = 2)]
	Decrement,

	#[token(";")]
	SemiColon,
	#[token(",")]
	Comma,
	#[token("=")]
	Eq,
	#[token("::")]
	DoubleColon,
	#[token(":")]
	Colon,
	#[token(".")]
	Point,
	#[token("(")]
	LParen,
	#[token(")")]
	RParen,
	#[token("[")]
	LBracket,
	#[token("]")]
	RBracket,
	#[token("{")]
	LBrace,
	#[token("}")]
	RBrace,
	#[token("<")]
	LChevron,
	#[token(">")]
	RChevron,
	#[token("!")]
	ExclamationMark,
	#[token("~")]
	Tilde,
	#[token("~=")]
	TildeEq,
	#[token("->")]
	Arrow,
	#[token("=>")]
	FatArrow,
	#[regex("//[^\n]*\n", logos::skip)]
	Comment,
	#[token("/*", block_comment)]
	BlockComment // TODO: doc comment
}

fn block_comment(lex: &mut Lexer<Token>) -> FilterResult<(), ()> {
	// let _asterik = lex.find(|x| {
	// 	println!("{:?}", x);
	// 	match x {
	// 	Ok(Token::BlockCommentEnd) => true,
	// 	_ => false
	// }});
	let mut lex = lex.peekable();
	loop {
		lex.find(|x| matches!(x, Ok(Token::Asterisk)));
		match lex.peek() {
			Some(Ok(Token::Slash)) => return FilterResult::Skip,
			None => return FilterResult::Error(()),
			_ => ()
		}
	}
}

// TODO: add more tests
#[cfg(test)]
mod tests {
	use crate::lexer::Token;
	use logos::Logos;

	#[test]
	fn test_lex() {
		let mut lex = Token::lexer("let abcd = 2 + 3;");
		assert_eq!(lex.next(), Some(Ok(Token::Let)));
		assert_eq!(lex.slice(), "let");

		assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
		assert_eq!(lex.slice(), "abcd");

		assert_eq!(lex.next(), Some(Ok(Token::Eq)));
		assert_eq!(lex.slice(), "=");

		assert_eq!(lex.next(), Some(Ok(Token::Int)));
		assert_eq!(lex.slice(), "2");

		assert_eq!(lex.next(), Some(Ok(Token::Plus)));
		assert_eq!(lex.slice(), "+");

		assert_eq!(lex.next(), Some(Ok(Token::Int)));
		assert_eq!(lex.slice(), "3");

		assert_eq!(lex.next(), Some(Ok(Token::SemiColon)));
		assert_eq!(lex.slice(), ";");
	}
}
