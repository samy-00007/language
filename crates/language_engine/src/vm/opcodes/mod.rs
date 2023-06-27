// TODO: accumulator
// TODO: maybe no need for jmp, just instructions for loops

// TODO: maybe do graph coloring for register allocation

pub type Reg = u8;
pub type Lit = i64;
pub type Address = u16;

// from https://github.com/boa-dev/boa/blob/main/boa_engine/src/vm/opcode/mod.rs
macro_rules! generate_impl {
	(
		$(#[$outer:meta])*
        pub enum $Type:ident {
            $(
                $(#[$inner:ident $($args:tt)*])*
                $Variant:ident
            ),*
        }
	) => {
		$(#[$outer])*
        pub enum $Type {
            $(
                $(#[$inner $($args)*])*
                $Variant
            ),*
        }

		impl From<u8> for $Type {
			#[inline]
			#[allow(non_upper_case_globals)]
			fn from(value: u8) -> Self {
				$(
					const $Variant: u8 = $Type::$Variant as u8;
				)*
				match value {
					$($Variant => Self::$Variant,)*
					_ => unreachable!()
				}
			}
		}


	};
}

generate_impl! {
	#[derive(Debug, Clone, Copy, PartialEq, Eq)]
	pub enum Opcode {
		/// Halts the Vm.
		Halt,
		/// Does nothing.
		Nop,
		/// Loads a literal into a register.
		/// TODO: other loads for different values
		///
		/// operands: `Reg`, `Lit`
		Load,
		/// Loads `true` in the first operand.
		///
		/// operands: `Reg`
		LoadTrue,
		/// Loads `false` in the first operand.
		///
		/// operands: `Reg`
		LoadFalse,
		/// Loads the second operand as a float in the first operand.
		///
		/// operands: `Reg`, `f64`
		LoadFloat,
		/// Loads the function in the second operand into the first operand.
		///
		/// operands: `Reg`, `u16`
		LoadF,
		/// Copies the value in the second operand into the first.
		///
		/// operands: `Reg`, `Reg`
		Move,
		/// Jumps to the address in the first operand.
		///
		/// operands: `Address`
		Jmp,
		/// Jumps to the address in the second operand if the value in the first operand is `true`.
		///
		/// operands: `Reg`, `Address`
		JmpIfTrue,
		/// Jumps to the address in the second operand if the value in the first operand is `false`.
		///
		/// operands: `Reg`, `Address`
		JmpIfFalse,
		/// Adds the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Reg`
		Add,
		/// Subtracts the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Reg`
		Sub,
		/// Multiplies the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Reg`
		Mul,
		/// Divides the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Reg`
		Div,
		/// If the value in the second operand in less than the one in the third, put `true` in the first operand.
		///
		/// operands: `Reg`, `Reg`, `Reg`
		Lt,
		/// Adds the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Lit`
		Addl,
		/// Subtracts the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Lit`
		Subl,
		/// Multiplies the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Lit`
		Mull,
		/// Divides the values in the second and third operand and puts the result in the first.
		///
		/// operands: `Reg`, `Reg`, `Lit`
		Divl,
		/// If the value in the second operand in less than the one in the third, put `true` in the first operand.
		///
		/// operands: `Reg`, `Reg`, `Lit`
		Ltl,
		// Cmp,
		/// Calls the function in the first operand, with as arguments the `nargs` registers following and returns `nret` values (similar to lua).
		///
		/// with B the number of args and C the number of return values:
		/// R\[A\], R[A+1], ..., R[A+C-1] = R\[A\](R[A+1], R[A+2], ..., R[A+B])
		///
		/// operands: `Reg`, nargs: `u8`, nret: `u8`
		Call,
		/// Returns from the current function. Returns the n-1 registers after the first operand.
		///
		/// return R\[A\], ..., R[A+B-1]
		///
		/// operands: `Reg`, `u8`
		Ret,
		/// operands: `Reg`
		Clock,
		/// operands: `Reg`
		Print
}
}
