use bio::{FASTQVec};
use std::env::args;
fn main() {
	if args().len() != 2 {
		panic!("Please provide file as argument");
	}

	let file_content = std::fs::read_to_string(&args().nth(1).unwrap()).unwrap();
	let (intg, fqv) = file_content.split_once('\n').unwrap();

	let thresh: f64 = intg.trim().parse().unwrap();

	println!("Quality threshold: {}", thresh);

	let fqvec = FASTQVec::from_string(fqv);
	let mut count = 0;
	for f in fqvec {
		let avg = f.average_quality().unwrap();
		if avg < thresh {
			println!("{}: {}", f.name, avg);
			count += 1;
		}
	}
	println!("{}", count);
}
