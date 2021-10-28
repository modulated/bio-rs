use bio::{FASTQVec};
use std::env::args;
fn main() {
	if args().len() != 2 {
		panic!("Please provide file as argument");
	}

	let file_content = std::fs::read_to_string(&args().nth(1).unwrap()).unwrap();
	let (intgs, fqv) = file_content.split_once('\n').unwrap();
	let ints: Vec<&str> = intgs.split(' ').collect();

	let thresh: u8 = ints[0].trim().parse().unwrap();
	let perc: u8 = ints[1].trim().parse().unwrap();

	println!("Quality threshold: {}", thresh);
	println!("Percent threshold: {}", perc);

	let fqvec = FASTQVec::from_string(fqv);
	let mut count = 0;
	for f in fqvec {		
		if f.is_quality_ok(thresh, perc).unwrap() {			
			count += 1;
		}
	}
	println!("{}", count);
}