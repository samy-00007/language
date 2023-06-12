pub trait Stack {
	type Value;

	fn append(&mut self, other: &[Self::Value]);
	fn push(&mut self, val: Self::Value);
	fn pop(&mut self) -> Self::Value;
	fn get(&self, i: usize) -> Self::Value;
	fn get_mut(&mut self, i: usize) -> &mut Self::Value;
	fn set(&mut self, i: usize, val: Self::Value);
	
	fn last(&self) -> Self::Value;
	fn last_mut(&mut self) -> &mut Self::Value;
	
	fn remove(&mut self, n: usize);
	fn reset(&mut self);

	fn len(&self) -> usize;
}
