use std::fmt::Display;
use crate::{Stmt, Expr, Literal, Prefix, Operator, Generic, Item, Ty, Argument};


fn _print_surrounded_vec<T: Display>(vec: &[T], sep: &str, surround_l: &str, surround_r: &str) -> String {
	let s = print_vec_with_sep(vec, sep);
	if s.is_empty() {
		s
	} else {
		format!("{surround_l}{s}{surround_r}")
	}
}

fn print_vec_with_sep<T: Display>(vec: &[T], sep: &str) -> String {
	vec.iter()
		.map(std::string::ToString::to_string)
		.collect::<Vec<String>>()
		.join(sep)
}

impl Display for Ty {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Ident(x) => x.to_owned(),
			Self::None => String::new()
		};
		write!(f, "{res}")
	}
}

impl Display for Argument {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}: {}", self.name, self.ty)
	}
}

impl Display for Item {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Constant { name, ty, value } => {
				let ty = ty.to_string();
				let ty = if ty.is_empty() { ty } else { format!(": {}", ty)};
				format!("const {} {} = {};", name, ty, value)
			},
			Self::Struct { name, fields } => format!("struct {} {{\n{}\n}}", name, print_vec_with_sep(fields, ",\n")),
			Self::Function { name, args, ty, block } => {
				let ty = ty.to_string();
				let ty = if ty.is_empty() { ty } else { format!("-> {}", ty)};
				format!("fn {}({}) {} {{\n{}\n}}", name, print_vec_with_sep(args, ", "), ty, print_vec_with_sep(block, "\n"))
			}
		};
		write!(f, "{res}")
	}
}

impl Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Expr(x) => format!("{x};"),
			Self::FnReturn(x) => format!("return {x}"),
			Self::If { cond, block } => format!("if ({}) {{\n{}\n}}", cond, print_vec_with_sep(block, "\n")),
			Self::Local { name, ty: t, val } => {
				let t_ = t
					.as_ref()
					.map_or_else(String::new, |t| format!(": {}", t));
				format!("let {name}{t_} = {val};")
			}
			Self::Return(x) => format!("{x}"),
			Self::Item(x) => x.to_string(),
			Self::While { cond, block } => {
				format!("while ({}) {{\n{}\n}}", cond, print_vec_with_sep(block, "\n"))
			}
			Self::Error => "<STMT ERROR>".to_string()
		};
		write!(f, "{res}")
	}
}

impl Display for Expr {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Block(x) => format!("{{\n{}\n}}", print_vec_with_sep(x, "\n")),
			Self::FnNamedCall { name, args } => format!("{name}({})", print_vec_with_sep(args, ", ")),
			Self::Ident(s) => s.to_string(),
			Self::Lit(l) => format!("{l}"),
			Self::Infix { op, lhs, rhs } => format!("({lhs} {op} {rhs})"),
			Self::Prefix(prefix, e) => format!("({prefix}{e})"),
			Self::FnCall { expr, args } => format!("{}({})", expr, print_vec_with_sep(args, ", ")),
			Self::Error => "<EXPR ERROR>".to_string()
		};
		write!(f, "{res}")
	}
}

impl Display for Literal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Bool(x) => x.to_string(),
			Self::Float(x) => x.to_string(),
			Self::Int(x) => x.to_string(),
			Self::String(x) => format!("\"{x}\"")
		};
		write!(f, "{res}")
	}
}

impl Display for Prefix {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::BitNot => "~",
			Self::Not => "!",
			Self::Minus => "-",
			Self::Plus => "+",
			Self::Err => "<PREFIX ERROR>"
		};
		write!(f, "{res}")
	}
}

impl Display for Operator {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let res = match self {
			Self::Assign => "=",

			Self::Add => "+",
			Self::AddEq => "+=",
			Self::Sub => "-",
			Self::SubEq => "-=",
			Self::Mul => "*",
			Self::MulEq => "*=",
			Self::Exponent => "**",
			Self::ExponentEq => "**=",
			Self::Div => "/",
			Self::DivEq => "/=",
			Self::Rem => "%",
			Self::RemEq => "%=",
			Self::Not => "!",

			Self::BitAnd => "&",
			Self::BitAndEq => "&=",
			Self::BitOr => "|",
			Self::BitOrEq => "|=",
			Self::BitNot => "~",
			Self::BitNotEq => "~=",
			Self::BitXor => "^",
			Self::BitXorEq => "^=",
			Self::LShift => "<<",
			Self::LShiftEq => "<<=",
			Self::RShift => ">>",
			Self::RShiftEq => ">>=",

			Self::Eq => "==",
			Self::Gt => ">",
			Self::Gte => ">=",
			Self::Lt => "<",
			Self::Lte => "<=",
			Self::Neq => "!=",
			Self::And => "&&",
			Self::AndEq => "&&=",
			Self::Or => "||",
			Self::OrEq => "||="
		};

		write!(f, "{res}")
	}
}


impl Display for Generic {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let traits = self.traits.join(" + ");
		let traits = if traits.is_empty() {
			traits
		} else {
			": ".to_string() + traits.as_str()
		};
		write!(f, "{}{}", self.name, traits)
	}
}