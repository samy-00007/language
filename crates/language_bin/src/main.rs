#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::inline_always)]
#![feature(core_intrinsics)]

use language_codegen::{compiler::Compiler, visitor::get_bytecode};
use language_engine::vm::Vm;
use language_parser::parser::Parser;

// #![feature(test)]
// mod bench;

fn main() {
	const __CODE: &str = "
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

	const _CODE: &str = "
	let i: number = 0;
	let str: string = \"\";
	while(i < 10) {
		str = str + \"test \";
		i = i + 1;
	}
	print(str);
	";

	let mut compiler = Compiler::new();

	let mut parser = Parser::new(CODE);

	let res = parser.parse();
	assert!(res.1.is_empty());

	let program = compiler.compile(res.0);

	println!("{program:?}\n\n");

	println!("{}", get_bytecode(&program));

	let mut vm = Vm::new(program);
	vm.run();
}

/*
   TODO: optimize everything (lexer, parser, ...)
*/
