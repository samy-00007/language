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
use execute::stack_bytecode::{
	vm::Vm,
	compiler::compile_block
};

/* 
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
*/


fn main() {
	/*let constants = vec![Literal::String("abcd".to_string()), Literal::Float(7.5)];
	let p = vec![
		Opcode::Const,
		Opcode::Const, // 1
		Opcode::DefGlob,
		Opcode::Hlt,
		Opcode::Hlt
	].into_iter().map(std::convert::Into::into).collect();
	let mut prog = Program::new(p, constants);

	prog.run();

	println!("{prog:?}");*/

	let mut parser = Parser::new("
	let a = 1;
	a = a + 2 * 3;
	print(a);
	");
	let parsed = parser.parse().unwrap();
	let res = compile_block(parsed);

	println!("{res:?}");

	let mut prog = Vm::new(res.0, res.1);

	prog.run();
	println!("{prog:?}");
}