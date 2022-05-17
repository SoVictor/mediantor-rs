pub trait Mediantor {
    fn insert(&mut self, x: i32);
    fn take(&mut self) -> i32;
    fn size(&self) -> usize;
}
