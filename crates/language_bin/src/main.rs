/*

#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::inline_always)]
#![feature(core_intrinsics)]

use language_codegen::compiler::Compiler;
use language_engine::vm::Vm;
use language_parser::parser::Parser;

// #![feature(test)]
// mod bench;

fn main() {
	const _CODE: &str = "
	fn fib(n: number) {
		if (n < 2) {
			return n
		}
		fib(n - 1) + fib(n - 2)
	}

	print(fib(14));
	";

	const CODE: &str = "
	let t: number = clock();
	let i: number = 0;
	while(clock() - t < 1000) {
		i = i + 1;
	}
	print(i);
	";

	let mut compiler = Compiler::new();

	let mut parser = Parser::new(CODE);

	let res = parser.parse();
	assert!(res.1.is_empty());

	let program = compiler.compile(res.0);

	println!("{program:?}");

	let mut vm = Vm::new(program);
	vm.run();
}
*/
/*
   TODO: optimize everything (lexer, parser, ...)
*/

fn main(){}