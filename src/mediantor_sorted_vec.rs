use crate::mediantor::Mediantor;

pub struct MediantorSortedVec {
	elements: Vec<i32>
}

impl MediantorSortedVec {
	pub fn new() -> Self {
		Self{elements: Vec::new()}
	}
}

impl Mediantor for MediantorSortedVec {
	// O(N).
	fn insert(&mut self, x: i32) {
		self.elements.push(x);
		for i in (1..self.elements.len()).rev() {
			if self.elements[i] < self.elements[i - 1] {
				self.elements.swap(i, i - 1);
			}
		}
	}

	// O(N).
	fn take(&mut self) -> i32 {
		let idx: usize = (self.elements.len() - 1) / 2;
		let ans = self.elements[idx];
		for i in idx..self.elements.len() - 1 {
			self.elements.swap(i, i + 1);
		}
		self.elements.pop();
		return ans;
	}
  
	// O(1).
	fn size(&self) -> usize {
		return self.elements.len();
	}
}
