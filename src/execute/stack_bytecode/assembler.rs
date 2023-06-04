#![allow(dead_code)]

use super::Opcode;

macro_rules! add_n {
	($n:ident, $t:ty) => {
		pub fn $n(&mut self, n: $t) {
			self.0.append(&mut n.to_be_bytes().to_vec());
		}
	};
}

pub struct Assembler(pub Vec<u8>);

impl Assembler {
	pub const fn new() -> Self {
		Self(Vec::new())
	}

	add_n!(add_u8, u8);
	add_n!(add_u16, u16);
	add_n!(add_u32, u32);
	add_n!(add_u64, u64);

	pub fn add_opcode(&mut self, op: Opcode) {
		self.0.push(op.into());
	}

	pub fn set_u8(&mut self, n: u8, i: usize) {
		self.0[i] = n;
	}
}
