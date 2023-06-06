#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::inline_always)]

mod execute;
mod lexer;
mod parser;

use parser::Parser;

use execute::register_bytecode::{assembler::Assembler, vm::Vm, Instr, JmpMode};

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

	assembler.add_instr(Instr::Clock(0));
	assembler.add_instr(Instr::Load(1, 1));
	assembler.add_instr(Instr::Load(4, 1000));
	assembler.add_instr(Instr::Jmp(JmpMode::RelativeForward, 4));
	assembler.add_instr(Instr::Add { reg_1: 2, reg_2: 1, dst: 2 });
	assembler.add_instr(Instr::Clock(3));
	assembler.add_instr(Instr::Sub { reg_1: 3, reg_2: 0, dst: 3 });
	assembler.add_instr(Instr::Cmp(3, 4));
	assembler.add_instr(Instr::Jlt(JmpMode::Absolute, 14));
	assembler.add_instr(Instr::Halt);

	let program = assembler.program;

	let mut vm = Vm::new(program);
	vm.run();

	println!("{:?}", vm.registers);
}
