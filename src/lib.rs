pub mod mediantor {
	use std::collections::BinaryHeap;
	use std::cmp::Reverse;
	
	pub trait Mediantor {
		fn insert(&mut self, x: i32);
		fn take(&mut self) -> i32;
		fn size(&self) -> usize;
	}

	pub struct MediantorSortedVec {
		elements: Vec<i32>
	}

	impl MediantorSortedVec {
		pub fn new() -> Self {
			Self{elements: Vec::new()}
		}
	}

	impl Mediantor for MediantorSortedVec {
		fn insert(&mut self, x: i32) {
			self.elements.push(x);
			for i in (1..self.elements.len()).rev() {
				if self.elements[i] < self.elements[i - 1] {
					self.elements.swap(i, i - 1);
				}
			}
		}

		fn take(&mut self) -> i32 {
			let idx: usize = (self.elements.len() - 1) / 2;
			let ans = self.elements[idx];
			for i in idx..self.elements.len() - 1 {
				self.elements.swap(i, i + 1);
			}
			self.elements.pop();
			return ans;
		}
	  
		fn size(&self) -> usize {
			return self.elements.len();
		}
	}
	
	pub struct MediantorHeap {
		lower_half: BinaryHeap<i32>,
		upper_half: BinaryHeap<Reverse<i32>>
	}
	
	impl MediantorHeap {
		pub fn new() -> Self {
			Self{lower_half: BinaryHeap::new(), upper_half: BinaryHeap::new()}
		}
		
		fn maybe_balance(&mut self) {
			if self.lower_half.len() < self.upper_half.len() {
				self.lower_half.push(self.upper_half.pop().unwrap().0);
			}
			else if self.lower_half.len() > self.upper_half.len() + 1 {
				self.upper_half.push(Reverse(self.lower_half.pop().unwrap()));
			}
		}
	}
	
	impl Mediantor for MediantorHeap {
		fn insert(&mut self, x: i32) {
			if self.lower_half.len() == 0 {
				self.lower_half.push(x);
				return;
			}
			
			if x < *self.lower_half.peek().unwrap() {
				self.lower_half.push(x);
			}
			else {
				self.upper_half.push(Reverse(x));
			}
			
			self.maybe_balance();
		}

		fn take(&mut self) -> i32 {
			let ans = self.lower_half.pop().unwrap();
			
			self.maybe_balance();
			
			return ans;
		}
	  
		fn size(&self) -> usize {
			return self.lower_half.len() + self.upper_half.len();
		}
	}
}
