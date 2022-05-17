use crate::mediantor::Mediantor;
use std::collections::VecDeque;

pub struct MediantorSqrtDecomp {
	buckets: Vec<VecDeque<i32>>,
	size: usize,
	bucket_size: usize
}

impl MediantorSqrtDecomp {
	fn usize_sqrt(x: usize) -> usize {
		(x as f64).sqrt() as usize
	}

	pub fn new(max_size: usize) -> Self {
		Self{buckets: Vec::new(), size: 0, bucket_size: Self::usize_sqrt(max_size).max(1)}
	}
}

impl Mediantor for MediantorSqrtDecomp {
	// O(sqrt(N)).
	fn insert(&mut self, x: i32) {
		if self.size == 0 {
			self.buckets.push([x].into());
			self.size += 1;
			return;
		}

		let mut bucket_idx: usize = self.buckets.len() - 1;
		while bucket_idx > 0 && self.buckets[bucket_idx].front().unwrap() > &x {
			bucket_idx -= 1;
		}
		let bucket = &mut self.buckets[bucket_idx];

		bucket.push_front(x);
		for i in 1..bucket.len() {
			if bucket[i] < bucket[i - 1] {
				bucket.swap(i, i - 1);
			}
		}

		for i in bucket_idx..self.buckets.len() - 1 {
			let t = *self.buckets[i].back().unwrap();
			self.buckets[i].pop_back();
			self.buckets[i + 1].push_front(t);
		}
		if self.buckets.last().unwrap().len() > self.bucket_size {
			let t = *self.buckets.last_mut().unwrap().back().unwrap();
			self.buckets.last_mut().unwrap().pop_back();
			self.buckets.push([t].into());
		}

		self.size += 1;
	}

	// O(sqrt(N)).
	fn take(&mut self) -> i32 {
		let idx = (self.size - 1) / 2;
		let bucket_idx = idx / self.bucket_size;
		let idx_in_bucket = idx % self.bucket_size;
		let bucket = &mut self.buckets[bucket_idx];

		let ans = bucket[idx_in_bucket];
		for i in idx_in_bucket..bucket.len() - 1 {
			bucket.swap(i, i + 1);
		}
		bucket.pop_back();

		for i in bucket_idx..self.buckets.len() - 1 {
			let t = *self.buckets[i + 1].front().unwrap();
			self.buckets[i + 1].pop_front();
			self.buckets[i].push_back(t);
		}
		if self.buckets.last().unwrap().len() == 0 {
			self.buckets.pop();
		}

		self.size -= 1;
		return ans;
	}
  
	// O(1).
	fn size(&self) -> usize {
		return self.size;
	}
}
