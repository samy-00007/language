use logos::{FilterResult, Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
#[logos(skip r"[ \t\n]+")]
pub enum Token {
	#[regex("[A-Za-z_][A-Za-z_0-9]*")] // TODO: maybe support unicode ?
	Identifier,
	#[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
	String,
	#[regex(r"[0-9][0-9_]*(\.[0-9_]+)([eE][\+-]?[0-9_]+)?", priority = 2)] // TODO: expand that
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

	#[token("true")]
	True,
	#[token("false")]
	False,

	// operators
	#[token("+")]
	Plus,
	#[token("+=")]
	PlusEq,
	#[token("-")]
	Minus,
	#[token("-=")]
	MinusEq,
	#[token("*")]
	Asterisk,
	#[token("*=")]
	AsteriskEq,
	#[token("/")]
	Slash,
	#[token("/=")]
	SlashEq,
	#[token("%")]
	Percent,
	#[token("%=")]
	PercentEq,
	
	// bitwise operations
	#[token("&")]
	Anpersand,
	#[token("&=")]
	AnpersandEq,
	#[token("|")]
	Bar,
	#[token("|=")]
	BarEq,
	#[token("^")]
	Caret,
	#[token("^=")]
	CaretEq,
	#[token("<<")]
	LShift,
	#[token("<<=")]
	LShiftEq,
	#[token(">>")]
	RShift,
	#[token(">>=")]
	RShiftEq,
	
	// comparaison operator
	#[token(">=")]
	Gte,
	#[token("<=")]
	Lte,
	#[token("==")]
	Eq,
	#[token("!=")]
	Neq,
	#[token("&&")]
	And,
	#[token("&&=")]
	AndEq,
	#[token("||")]
	Or,
	#[token("||=")]
	OrEq,
	
	#[token("++")]
	Increment,
	#[token("--")]
	Decrement,
	
	
	#[token(";")]
	SemiColon,
	#[token(",")]
	Comma,
	#[token("=")]
	Assign,
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
	#[regex("//[^\n]*\n", logos::skip)]
	Comment,
	#[token("/*", block_comment)]
	BlockComment
	// TODO: doc comment
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

		assert_eq!(lex.next(), Some(Ok(Token::Assign)));
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
