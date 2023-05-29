// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]

mod lexer;
mod parser;
mod execute;


use parser::Parser;


fn main() {
		let mut parser = Parser::new("
	let t = clock();
	let i = 0;
	while(clock() - t < 1000) {
		i = i + 1;
	}
	print(i);
");
	let parsed = parser.parse();
	
	let compiled = execute::stack_bytecode::compiler::compile(parsed.0); // 1.6M


	let mut vm = execute::stack_bytecode::vm::Vm::new(compiled.0, compiled.1);

	vm.run();

	// execute::walker::walk(parsed.0, &mut locals, args); // 420k
	// execute::stack_bytecode::compiler::compile(parsed.0); // 1.680M
	// js: 18.8M
	// python: 6M	
}