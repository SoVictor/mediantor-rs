use std::io;

fn main() {
	use mediantor_rs::*;
	
	let mut implementation: Option<MediantorImplementation> = None;
	while implementation.is_none() {
		println!("Please choose which Mediantor implementation to use by writing a single number: ");
		println!("0 - Mediantor as two heaps");
		println!("1 - Mediantor as SQRT-decomposition");
		println!("2 - Mediantor as a sorted vector");

		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		let implementation_idx: usize = input.trim().parse::<usize>().expect("Failed to parse number");

		implementation = match implementation_idx {
			0 => Some(MediantorImplementation::Heap),
			1 => Some(MediantorImplementation::SqrtDecomp),
			2 => Some(MediantorImplementation::SortedVec),
			_ => None,
		}
	}
	let implementation: MediantorImplementation = implementation.unwrap();

	println!();
	println!("Please provide an input in a format described in README:");

	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse::<usize>().expect("Failed to parse n");

	let mut mediantor = create_mediantor(implementation, n);
	
	let mut answer: Vec<i32> = Vec::new();
	for _i in 0..n {
		let mut input = String::new();
		io::stdin().read_line(&mut input).expect("Failed to read line");
		let mut it = input.split_whitespace();
		
		let operation: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse operation");
		if operation == 1 {
			let x: i32 = it.next().unwrap().parse::<i32>().expect("Failed to parse x");
			mediantor.insert(x);
		}
		else {
			answer.push(mediantor.take());
		}
	}
	
	println!();
	println!("Output:");
	for x in answer {
		println!("{}", x);
	}
}
