use language_engine::vm::opcodes::Reg;
use std::collections::HashMap;

use super::utils::*;

#[derive(Debug, Default)]
pub struct Env {
	functions: HashMap<String, Func>,
	variables: HashMap<String, Var>,
	last_reg: Reg
}

impl Env {
	pub fn allocate_reg(&mut self) -> Reg {
		assert!(self.last_reg < Reg::MAX);
		self.last_reg += 1;
		self.last_reg - 1
	}

	pub fn free_last_reg(&mut self) {
		assert!(self.last_reg > 0);
		self.last_reg -= 1;
	}

	pub fn add_var(&mut self, name: String, ty: Type) -> Reg {
		assert!(!self.has_var(&name));
		let reg = self.allocate_reg();
		self.variables.insert(name, Var::new(reg, ty));
		reg
	}

	pub fn has_var(&mut self, name: &str) -> bool {
		self.variables.contains_key(name)
	}

	pub fn get_var_reg(&mut self, name: &str) -> Var {
		*self.variables.get(name).unwrap()
	}

	pub fn get_function(&mut self, name: &str) -> Func {
		*self.functions.get(name).unwrap()
	}

	pub fn set_function(&mut self, name: String, f: Func) {
		self.functions.insert(name, f);
	}
}
