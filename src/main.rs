use std::io;

mod mediantor {
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

fn main() {
	use crate::mediantor::Mediantor;
	use crate::mediantor::MediantorHeap;
	
	let mut mediantor = MediantorHeap::new();
	
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse::<i32>().expect("Failed to parse n");
	
	let mut answer: Vec<i32> = Vec::new();
	for _i in 0..n {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		let mut it = input.split_whitespace();
		
		let operation: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse operation");
		if operation == 1 {
			let x: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse x");
			mediantor.insert(x);
		}
		else {
			answer.push(mediantor.take());
		}
	}
	
	println!();
	println!("Output:");
	for x in answer {
		println!("{}", x);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
    #[test]
    fn trivial() {
		let mut mediantor = MediantorSortedVec{elements: Vec::new()};
		mediantor.insert(1);
		mediantor.insert(2);
		mediantor.insert(3);
		assert_eq!(mediantor.take(), 2);

		mediantor.insert(2);
		mediantor.insert(4);
		assert_eq!(mediantor.take(), 2);
    }
}
