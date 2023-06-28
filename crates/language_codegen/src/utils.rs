use language_ast::Ty;





#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Func {
	pub id: u16,
	pub ret_ty: Type,
	pub n_args: u8,
	pub n_ret: u8
}

impl Func {
	pub fn new(id: u16, ret_ty: Type, n_args: u8, n_ret: u8) -> Self {
		Self { id, ret_ty, n_args, n_ret }
	}
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Var {
	pub reg: u8,
	pub ty: Type
}

impl Var {
	pub const fn new(reg: u8, ty: Type) -> Self {
		Self { reg, ty }
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
	Bool,
	Number,
	String,
	None
}

impl From<Ty> for Type {
	fn from(value: Ty) -> Self {
		let Ty::Ident(ty) = value else {unreachable!()}; // TODO: handle that
		match ty.as_str() {
			"string" => Self::String,
			"number" => Self::Number,
			"bool" => Self::Bool,
			_ => unreachable!()
		}
	}
}
