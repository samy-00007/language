// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]
mod lexer;
mod parser;
mod execute;

use std::collections::HashMap;

// use lexer::Token;
// use logos::Logos;
use parser::Parser;

fn main() {
	
	// let mut parser = Parser::new(
	// 	"
	// 	let total = 0;
	// 	let count = arg_0;
	// 	while (count > 0) {
	// 		total = total + arg_1;
	// 		count = count - 1;
	// 	}
	// "
	// );
	let mut parser = Parser::new("let a = 5; print(-a);");
	for x in parser.parse().unwrap() {
		println!("{x}");
	}

	// let blk = parser.parse().unwrap();
	// let mut locals = HashMap::new();
	// let args = [10000, 13];
	// execute::walker::walk(blk, &mut locals, args);
	// println!("{:?}", locals);

}
