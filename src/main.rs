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
		let total = 0;
		let count = args_0;
		while (count > 0) {
			total = total + args_1;
			count = count - 1;
		}
	"
	);
	// println!("{:?}", lex.collect::<Vec<Result<Token, ()>>>());
	// let mut parser = Parser::new("let a = 5.5 + 3;");
	for x in parser.parse().unwrap() {
		println!("{}", x);
	}
	

}
