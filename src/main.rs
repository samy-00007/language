// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]

mod lexer;
mod parser;
mod execute;

use parser::Parser;
use execute::stack_bytecode::{
	vm::Vm,
	compiler::compile
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
	let mut parser = Parser::new("
	let a = 1;
	let b = a + 2 * 3;
	{
		let c = 25;
		print(a + b + c);
	};
	print(a * b);
	");
	let parsed = parser.parse().unwrap();
	let res = compile(parsed);

	println!("{res:?}");

	let mut prog = Vm::new(res.0, res.1);

	prog.run();
	println!("{prog:?}");
}