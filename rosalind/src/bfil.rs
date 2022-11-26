use bio::FASTQVec;
use std::env::args;
fn main() {
	if args().len() != 2 {
		panic!("Please provide file as argument");
	}

	let file = std::fs::read_to_string(args().nth(1).unwrap()).unwrap();
	let (thresh, data) = file.split_once('\n').unwrap();
	let thresh: u8 = thresh.trim().parse().unwrap();

	let mut fqvec = FASTQVec::from_string(data);
	for f in fqvec.iter_mut() {
		f.trim_quality(thresh);
		println! {"{}", &f};
	}
}
