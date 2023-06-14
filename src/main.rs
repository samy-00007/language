#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::inline_always)]

mod execute;
mod lexer;
mod utils;
// mod parser;

// use parser::Parser;

use execute::register_bytecode::{assembler::Assembler, vm::Vm, Instr, JmpMode};

use crate::execute::register_bytecode::Address;

// #![feature(test)]
// mod bench;

fn main() {
	/*
	   let t = clock();
	   let i = 0;
	   while(clock() - t < 1000) {
		   i = i + 1;
	   }
	   print(i);
	*/
	let mut assembler = Assembler::new();

	/*assembler.add_instr(Instr::Clock(0));
	assembler.add_instr(Instr::Load(1, 1));
	assembler.add_instr(Instr::Load(4, 1000));
	assembler.add_instr(Instr::Jmp(JmpMode::RelativeForward, 4));
	let add = assembler.add_instr(Instr::Add {
		reg_1: 2,
		reg_2: 1,
		dst: 2
	});
	assembler.add_instr(Instr::Clock(3));
	assembler.add_instr(Instr::Sub {
		reg_1: 3,
		reg_2: 0,
		dst: 3
	});
	assembler.add_instr(Instr::Cmp(3, 4));
	assembler.add_instr(Instr::Jlt(JmpMode::Absolute, add as Address));
	assembler.add_instr(Instr::Print(2));
	assembler.add_instr(Instr::Halt);*/

	let mut func = Assembler::new();
	func.add_instr(Instr::Load(2, 2));

	func.add_instr(Instr::Cmp(0, 2));
	func.add_instr(Instr::Jge(JmpMode::RelativeForward, 3));
	func.add_instr(Instr::Ret(0, 1)); // return reg 0
	func.add_instr(Instr::Subl {
		reg_1: 0,
		val: 1,
		dst: 2
	});
	func.add_instr(Instr::Subl {
		reg_1: 0,
		val: 2,
		dst: 3
	});
	func.add_instr(Instr::LoadF(1, 0));
	func.add_instr(Instr::Call(1, 1, 1)); // reg 1 as param
	func.add_instr(Instr::LoadF(2, 0));
	func.add_instr(Instr::Call(2, 1, 1));
	func.add_instr(Instr::Add {
		reg_1: 2,
		reg_2: 1,
		dst: 0
	});
	func.add_instr(Instr::Ret(0, 1));
	assembler.add_function(func.program);

	assembler.add_instr(Instr::LoadF(0, 0));
	assembler.add_instr(Instr::Load(1, 3));
	assembler.add_instr(Instr::Call(0, 1, 1));
	assembler.add_instr(Instr::Print(0));
	assembler.add_instr(Instr::Halt);

	/*
	   fn fibonacci(n: number) -> number  {
		   if n < 2 {
			   n
		   } else {
			   fibonacci(n - 1) + fibonacci(n - 2)
		   }
	   }

	   fn fibonacci_(n: number) -> number {
		   let a: number = 0;
		   let b: number = 1;

		   if n < 2 {
			   return n
		   }

		   for _ in 0..n {
			   let _b = b;
			   b = a + b;
			   a = _b;
		   }
		   b
	   }

	   let a = fibonacci(14);
	   print(a)
	*/

	// core::intrinsics::unlikely

	let program = assembler.program;

	println!("{program:?}");

	let mut vm = Vm::new(program);
	vm.run();

	// println!("{:?}", vm.registers);
	// println!("{:?}", unsafe {vm.registers[2].val.int });
}

/*
   TODO: optimize everything (lexer, parser, ...)
*/
