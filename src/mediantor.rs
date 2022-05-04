pub mod mediantor {
	use std::collections::BinaryHeap;
	use std::cmp::Reverse;
	
	pub trait Mediantor {
		fn insert(&mut self, x: i32);
		fn take(&mut self) -> i32;
		fn size(&self) -> usize;
	}
}
