// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
// #![allow(clippy::all)]
// #![allow(dead_code, unused_imports)]
mod lexer;
mod parser;
mod execute;

use std::collections::HashMap;

// use lexer::Token;
// use logos::Logos;
use parser::Parser;

fn main() {
	
	let mut parser = Parser::new(
		"
		let total = 0;
		let count = arg_0;
		while (count > 0) {
			total = total + arg_1;
			count = count - 1;
		}
	"
	);
	// println!("{:?}", lex.collect::<Vec<Result<Token, ()>>>());
	// let mut parser = Parser::new("let a = 5.5 + 3;");
	// for x in parser.parse().unwrap() {
	// 	println!("{}", x);
	// }

	let blk = parser.parse().unwrap();
	let mut locals = HashMap::new();
	let args = [10000, 13];
	execute::walker::walk(blk, &mut locals, args);
	println!("{:?}", locals);

}
