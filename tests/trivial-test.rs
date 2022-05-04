#[cfg(test)]
mod tests {	
    #[test]
    fn trivial() {
		use mediantor_rs::mediantor::Mediantor;
		use mediantor_rs::mediantor::MediantorSortedVec;
	
		let mut mediantor = MediantorSortedVec::new();
		mediantor.insert(1);
		mediantor.insert(2);
		mediantor.insert(3);
		assert_eq!(mediantor.take(), 2);

		mediantor.insert(2);
		mediantor.insert(4);
		assert_eq!(mediantor.take(), 2);
    }
}
