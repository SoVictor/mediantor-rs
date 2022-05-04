mod mediantor;
pub use crate::mediantor::Mediantor;

mod mediantor_heap;
pub use crate::mediantor_heap::MediantorHeap;

mod mediantor_sorted_vec;
pub use crate::mediantor_sorted_vec::MediantorSortedVec;

mod mediantor_sqrt_decomp;
pub use crate::mediantor_sqrt_decomp::MediantorSqrtDecomp;

pub enum MediantorImplementation {
	MediantorHeap,
	MediantorSqrtDecomp,
	MediantorSortedVec,
}

pub fn create_mediantor(implementation: MediantorImplementation, max_size: usize) -> Box<dyn Mediantor> {
	match implementation {
		MediantorImplementation::MediantorHeap => Box::new(MediantorHeap::new()),
		MediantorImplementation::MediantorSqrtDecomp => Box::new(MediantorSqrtDecomp::new(max_size)),
		MediantorImplementation::MediantorSortedVec => Box::new(MediantorSortedVec::new()),
	}
}
