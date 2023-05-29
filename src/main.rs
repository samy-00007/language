// using https://domenicquirl.github.io/blog/parsing-basics/
// FIXME:
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
// #![warn(clippy::cargo)]

mod lexer;
mod parser;
mod execute;


use parser::Parser;

const CODE: &str = "let t = clock();
let i = 0;
while(clock() - t < 1000) {
	i = i + 1;
}
print(i);";



fn main() {
	println!("ast walk: ");
	ast_walk(false);
	println!();
	println!("===================");
	println!();
	println!("stack-based bytecode vm: ");
	stack_bytecode(false);
}


// js and python performance with the same code:
// 		- js: 		i = ~18.8M
// 		- python: 	i = ~6M

#[allow(clippy::tabs_in_doc_comments)]
/// Use ast-walk to run the code
/// From the little test:
/// 	- unoptimized:	i = ~420k
/// 	- optimized:	i = ~2.550M
/// And with [`rustc_hash::FxHashMap`]:
/// 	- unoptimized:	i = ~480k
/// 	- optimized:	i = ~3M
#[allow(dead_code)]
fn ast_walk(debug: bool) {
	// use std::collections::HashMap;
	use rustc_hash::FxHashMap as HashMap;
	use crate::execute::walker::walk;

	let mut parser = Parser::new(CODE);
	let parsed = parser.parse();

	if debug {
		println!("parsed: {parsed:?}");
	}

	let mut locals = HashMap::default();
	let args = [0;2];
	
	if debug {
		println!("args: {args:?}");
	}

	walk(parsed.0, &mut locals, args);
	if debug {
		println!("locals: {locals:?} ");
	}
}

#[allow(clippy::tabs_in_doc_comments)]
/// Use a bytecode stack-based VM to run the code
/// From the little test:
/// 	- unoptimized: 	i = ~1.6M
/// 	- optimized: 	i = ~10.5M
#[allow(dead_code)]
fn stack_bytecode(debug: bool) {
	use crate::execute::stack_bytecode::{compiler::compile, vm::Vm};

	let mut parser = Parser::new(CODE);
	let parsed = parser.parse();

	let compiled = compile(parsed.0.clone());

	if debug {
		println!("parsed: {parsed:?}");
		println!("(compiled, constants): {compiled:?}");
	}

	let mut vm = Vm::new(compiled.0, compiled.1);
	
	vm.run();
	
}