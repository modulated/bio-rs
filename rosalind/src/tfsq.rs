use bio::{BioResult, FASTAVec, FASTQVec};
use std::env::args;
fn main() -> BioResult<()> {
	if args().len() != 2 {
		panic!("Please provide file as argument");
	}

	let fqvec: FASTAVec = FASTQVec::from_file(&args().nth(1).unwrap()).into();
	for f in fqvec {
		println!(">{}", f.name);
		println!("{}", f.seq);
		println!();
	}
	Ok(())
}
