use crate::mediantor::Mediantor;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct MediantorHeap {
    lower_half: BinaryHeap<i32>,
    upper_half: BinaryHeap<Reverse<i32>>,
}

impl MediantorHeap {
    pub fn new() -> Self {
        Self {
            lower_half: BinaryHeap::new(),
            upper_half: BinaryHeap::new(),
        }
    }

    fn maybe_balance(&mut self) {
        if self.lower_half.len() < self.upper_half.len() {
            self.lower_half.push(self.upper_half.pop().unwrap().0);
        } else if self.lower_half.len() > self.upper_half.len() + 1 {
            self.upper_half
                .push(Reverse(self.lower_half.pop().unwrap()));
        }
    }
}

impl Default for MediantorHeap {
    fn default() -> Self {
        Self::new()
    }
}

impl Mediantor for MediantorHeap {
    // O(log N).
    fn insert(&mut self, x: i32) {
        if self.lower_half.is_empty() {
            self.lower_half.push(x);
            return;
        }

        if x < *self.lower_half.peek().unwrap() {
            self.lower_half.push(x);
        } else {
            self.upper_half.push(Reverse(x));
        }

        self.maybe_balance();
    }

    // O(log N).
    fn take(&mut self) -> i32 {
        let ans = self.lower_half.pop().unwrap();
        self.maybe_balance();
        ans
    }

    // O(1).
    fn size(&self) -> usize {
        self.lower_half.len() + self.upper_half.len()
    }
}
