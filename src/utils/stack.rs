pub trait Stack {
	type Value;

	fn push(&mut self, val: Self::Value);
	fn pop(&mut self) -> Self::Value;
	fn get(&mut self, i: usize) -> Self::Value;
	fn set(&mut self, i: usize, val: Self::Value);

	fn last(&self) -> Self::Value;
	fn last_mut(&mut self) -> &mut Self::Value;

	fn reset(&mut self);

	fn len(&self) -> usize;
}
