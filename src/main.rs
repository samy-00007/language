#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::inline_always)]

mod execute;
mod lexer;
mod parser;

use parser::Parser;

use execute::register_bytecode::{assembler::Assembler, vm::Vm, Instr};

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

	assembler.add_instr(Instr::Load(1, 10000));
	assembler.add_instr(Instr::Load(2, 13));
	assembler.add_instr(Instr::Load(3, 1));
	assembler.add_instr(Instr::Jmp(23));
	assembler.add_instr(Instr::Add {
		reg_1: 0,
		reg_2: 2,
		dst: 0
	});
	assembler.add_instr(Instr::Sub {
		reg_1: 1,
		reg_2: 3,
		dst: 1
	});
	assembler.add_instr(Instr::Cmp(1, 4));
	assembler.add_instr(Instr::Jgt(15));
	assembler.add_instr(Instr::Halt);

	let program = assembler.program;

	let mut vm = Vm::new(program);
	vm.run();

	println!("{:?}", vm.registers);
}
