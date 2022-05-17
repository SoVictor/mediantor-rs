mod mediantor;
pub use crate::mediantor::Mediantor;

mod mediantor_heap;
pub use crate::mediantor_heap::MediantorHeap;

mod mediantor_sorted_vec;
pub use crate::mediantor_sorted_vec::MediantorSortedVec;

mod mediantor_sqrt_decomp;
pub use crate::mediantor_sqrt_decomp::MediantorSqrtDecomp;

#[derive(Copy, Clone)]
pub enum MediantorImplementation {
    Heap,
    SqrtDecomp { max_size: usize },
    SortedVec,
}

pub fn create_mediantor(implementation: MediantorImplementation) -> Box<dyn Mediantor> {
    match implementation {
        MediantorImplementation::Heap => Box::new(MediantorHeap::new()),
        MediantorImplementation::SqrtDecomp { max_size } => {
            Box::new(MediantorSqrtDecomp::new(max_size))
        }
        MediantorImplementation::SortedVec => Box::new(MediantorSortedVec::new()),
    }
}
