// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
// #![allow(clippy::all)]
// #![allow(dead_code, unused_imports)]
mod lexer;
mod parser;

// use lexer::Token;
// use logos::Logos;
use parser::Parser;

fn main() {
	let mut parser = Parser::new(
		"
	let abcd = 2 + 3;
	let var_2: number = abcd * 5;
	let var3 = \"abcd\";
	print(var3 + var_2 + abdc);
	// abcd
	/*
	cihuchu
	uch. */
	if (abcd == 5) {
		print(true);
	}
	"
	);
	// println!("{:?}", lex.collect::<Vec<Result<Token, ()>>>());
	// let mut parser = Parser::new("let a = 5.5 + 3;");
	println!("{:?}", parser.parse());
}
