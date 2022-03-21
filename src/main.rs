trait Mediantor {
    fn insert(&mut self, x: i32);
  fn take(&mut self) -> i32;
  fn size(&self) -> usize;
}

struct MediantorSortedVec {
  elements: Vec<i32>
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

fn main() {
  let mut mediantor = MediantorSortedVec{elements: Vec::new()};
  mediantor.insert(1);
  mediantor.insert(2);
  mediantor.insert(3);
  println!("{}", mediantor.take());
  
  mediantor.insert(2);
  mediantor.insert(4);
  println!("{}", mediantor.take());
}
