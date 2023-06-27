use language_ast::Ty;

#[derive(Debug)]
pub struct Var {
	pub reg: u8,
	pub ty: Type
}

impl Var {
	pub const fn new(reg: u8, ty: Type) -> Self {
		Self { reg, ty }
	}
}

#[derive(Debug)]
pub enum Type {
	Bool,
	Number,
	String
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
