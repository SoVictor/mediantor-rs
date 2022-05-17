use mediantor_rs::*;
use parameterized::parameterized;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[parameterized(implementation = {
    MediantorImplementation::Heap,
	MediantorImplementation::SqrtDecomp,
	MediantorImplementation::SortedVec,
})]
fn on_data(implementation: MediantorImplementation) {
	for entry in fs::read_dir("tests/data").unwrap() {
		let input_path = entry.unwrap().path();
		if input_path.extension().unwrap() != "in" {
			continue;
		}

		let mut output_path = input_path.clone();
		output_path.set_extension("out");

		let input_file = File::open(input_path).unwrap();
		let mut input_file = BufReader::new(input_file);

		let output_file = File::open(output_path).unwrap();
		let mut output_file = BufReader::new(output_file);

		let mut input = String::new();
		input_file.read_line(&mut input).expect("Failed to read line");
		let n = input.trim().parse().expect("Failed to parse n");

		let mut mediantor = create_mediantor(implementation, n);

		for _i in 0..n {
			let mut input = String::new();
			input_file.read_line(&mut input).expect("Failed to read line");
			let mut it = input.split_whitespace();
		
			let operation: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse operation");
			if operation == 1 {
				let x: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse x");
				mediantor.insert(x);
			}
			else {
				let mut output = String::new();
				output_file.read_line(&mut output).expect("Failed to read line");
				let x: i32 = output.trim().parse::<i32>().expect("Failed to parse x");
				assert_eq!(mediantor.take(), x);
			}
		}

		let mut input = String::new();
		let bytes = input_file.read_line(&mut input).expect("Failed to read line");
		assert_eq!(bytes, 0);
	}
}
