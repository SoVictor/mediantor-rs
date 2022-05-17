use mediantor_rs::*;
use parameterized::parameterized;

#[parameterized(implementation = {
    MediantorImplementation::Heap,
	MediantorImplementation::SqrtDecomp{max_size: 4},
	MediantorImplementation::SortedVec,
})]
fn trivial(implementation: MediantorImplementation) {
    let mut mediantor = create_mediantor(implementation);
    mediantor.insert(1);
    mediantor.insert(2);
    mediantor.insert(3);
    assert_eq!(mediantor.take(), 2);

    mediantor.insert(2);
    mediantor.insert(4);
    assert_eq!(mediantor.take(), 2);
}
