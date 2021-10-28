use bio::FASTQVec;
use std::env::args;
fn main() {
	if args().len() != 2 {
		panic!("Please provide file as argument");
	}

	let file = std::fs::read_to_string(args().nth(1).unwrap()).unwrap();
	let (thresh, data) = file.split_once('\n').unwrap();
	let thresh: f64 = thresh.trim().parse().unwrap();

	let fqvec = FASTQVec::from_string(data);
	let qual = fqvec.position_base_quality();

	let res = qual
		.unwrap()
		.iter()
		.fold(0, |a, x| if x < &thresh { a + 1 } else { a });
	println!("{}", res);
}
