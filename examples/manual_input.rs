use std::io;

fn main() {
	use mediantor_rs::Mediantor;
	use mediantor_rs::MediantorHeap;
	
	let mut mediantor = MediantorHeap::new();
	
	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse::<i32>().expect("Failed to parse n");
	
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
