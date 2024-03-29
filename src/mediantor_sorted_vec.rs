use crate::mediantor::Mediantor;

pub struct MediantorSortedVec {
    elements: Vec<i32>,
}

impl MediantorSortedVec {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl Default for MediantorSortedVec {
    fn default() -> Self {
        Self::new()
    }
}

impl Mediantor for MediantorSortedVec {
    // O(N).
    fn insert(&mut self, x: i32) {
        let idx = self.elements.binary_search(&x).unwrap_or_else(|x| x);
        self.elements.insert(idx, x);
    }

    // O(N).
    fn take(&mut self) -> i32 {
        let idx = (self.elements.len() - 1) / 2;
        self.elements.remove(idx)
    }

    // O(1).
    fn size(&self) -> usize {
        self.elements.len()
    }
}
