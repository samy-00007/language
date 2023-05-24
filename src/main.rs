// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
#![allow(clippy::all)]
#![allow(dead_code, unused_imports)]
mod lexer;
mod parser;

use lexer::Token;
use logos::Logos;

fn main(){
	let lex = Token::lexer("
	let abcd = 2 + 3;
	let var_2: number = abcd * 5;
	let var3 = \"abcd\";
	console.log(var3 + var_2 + abdc);
	// abcd
	/*
	cihuchu
	uch. */
	if (abcd == 5) {
		console.log(true)
	}
	");

	println!("{:?}", lex.collect::<Vec<Result<Token, ()>>>());
}