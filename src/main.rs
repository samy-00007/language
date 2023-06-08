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
/*
	assembler.add_instr(Instr::Clock(0));
	assembler.add_instr(Instr::Load(1, 1));
	assembler.add_instr(Instr::Load(4, 1000));
	assembler.add_instr(Instr::Jmp(JmpMode::RelativeForward, 4));
	assembler.add_instr(Instr::Add {
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
	assembler.add_instr(Instr::Jlt(JmpMode::Absolute, 14));
	assembler.add_instr(Instr::Halt);
*/

	// TODO: pass floats to instr

	assembler.add_instr(Instr::Jmp(JmpMode::Absolute, 70));
	// fn add
	let add = assembler.add_instr(Instr::GetArg(0, 0));
	assembler.add_instr(Instr::Load(1, 2));
	assembler.add_instr(Instr::Cmp(0, 1));
	assembler.add_instr(Instr::Jge(JmpMode::RelativeForward, 2));
	assembler.add_instr(Instr::Ret(0));
	assembler.add_instr(Instr::Subl { reg_1: 0, val: 1, dst: 2 });
	assembler.add_instr(Instr::Subl { reg_1: 0, val: 2, dst: 1 });
	assembler.add_instr(Instr::Push(2));
	assembler.add_instr(Instr::Call(add as Address, 1));
	assembler.add_instr(Instr::Pop(0));
	assembler.add_instr(Instr::Push(1));
	assembler.add_instr(Instr::Call(add as Address, 1));
	assembler.add_instr(Instr::Pop(1));
	assembler.add_instr(Instr::Add { reg_1: 0, reg_2: 1, dst: 0 });
	assembler.add_instr(Instr::Ret(0));

	assembler.add_instr(Instr::Load(0, 14));
	assembler.add_instr(Instr::Push(0));
	assembler.add_instr(Instr::Call(add as Address, 1));
	assembler.add_instr(Instr::Pop(0));
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

	let a = fibonacci(14);
	print(a)
 */

	let program = assembler.program;

	println!("{program:?}");

	let mut vm = Vm::new(program);
	vm.run();


	// println!("{:?}", vm.registers);
	// println!("{:?}", unsafe {vm.registers[2].val.int });
}
