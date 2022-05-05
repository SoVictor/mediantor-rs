use mediantor_rs::*;
use parameterized::parameterized;

#[parameterized(implementation = {
    MediantorImplementation::MediantorHeap,
	MediantorImplementation::MediantorSqrtDecomp,
	MediantorImplementation::MediantorSortedVec,
})]
fn trivial(implementation: MediantorImplementation) {
	let mut mediantor = create_mediantor(implementation, 4);
	mediantor.insert(1);
	mediantor.insert(2);
	mediantor.insert(3);
	assert_eq!(mediantor.take(), 2);

	mediantor.insert(2);
	mediantor.insert(4);
	assert_eq!(mediantor.take(), 2);
}
