use std::io;

fn main() {
    use mediantor_rs::*;

    let implementation = loop {
        println!(
            "Please choose which Mediantor implementation to use by writing a single number: "
        );
        println!("0 - Mediantor as two heaps");
        println!("1 - Mediantor as SQRT-decomposition");
        println!("2 - Mediantor as a sorted vector");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let implementation_idx: usize = input
            .trim()
            .parse::<usize>()
            .expect("Failed to parse number");

        match implementation_idx {
            0 => break MediantorImplementation::Heap,
            1 => break MediantorImplementation::SqrtDecomp { max_size: 1 },
            2 => break MediantorImplementation::SortedVec,
            _ => continue,
        }
    };

    println!();
    println!("Please provide an input in a format described in README:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let n = input.trim().parse().expect("Failed to parse n");

    let implementation = match implementation {
        MediantorImplementation::SqrtDecomp { .. } => {
            MediantorImplementation::SqrtDecomp { max_size: n }
        }
        other => other,
    };
    let mut mediantor = create_mediantor(implementation);

    let mut answer: Vec<i32> = Vec::new();
    for _i in 0..n {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let mut it = input.split_whitespace();

        let operation: i32 = it
            .next()
            .unwrap()
            .parse()
            .expect("Failed to parse operation");
        if operation == 1 {
            let x: i32 = it.next().unwrap().parse().expect("Failed to parse x");
            mediantor.insert(x);
        } else {
            answer.push(mediantor.take());
        }
    }

    println!();
    println!("Output:");
    for x in answer {
        println!("{}", x);
    }
}
