use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n]+")]
pub enum Token {
	#[regex("[A-Za-z_][A-Za-z_0-9]*")]
	Identifier,
	#[regex(r#""([^"\\]|\\t|\\u|\\n|\\")*""#)]
    String,
	#[regex(r"-?[0-9][0-9_]*(\.[0-9_]+)?([eE][\+-]?[0-9_]+)?", priority = 2)]
	Number,


	#[token(";")]
	SemiColon,
	#[token("=")]
	Assign,
	#[token("==")]
	Eq,
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::lexer::Token;

	#[test]
	fn test_lex() {
		let mut lex = Token::lexer("let abcd = 2 + 3;");
		assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
		assert_eq!(lex.slice(), "let");

		assert_eq!(lex.next(), Some(Ok(Token::Identifier)));
		assert_eq!(lex.slice(), "abcd");

		assert_eq!(lex.next(), Some(Ok(Token::Assign)));
		assert_eq!(lex.slice(), "=");

		assert_eq!(lex.next(), Some(Ok(Token::Number)));
		assert_eq!(lex.slice(), "2");

		assert_eq!(lex.next(), Some(Ok(Token::Plus)));
		assert_eq!(lex.slice(), "+");

		assert_eq!(lex.next(), Some(Ok(Token::Number)));
		assert_eq!(lex.slice(), "3");

		assert_eq!(lex.next(), Some(Ok(Token::SemiColon)));
		assert_eq!(lex.slice(), ";");
	}
}