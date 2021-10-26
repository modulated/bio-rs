use bio::{compare_strings, transition_transversion_ratio, FASTAVec};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("TRAN Problem");
	if args().len() != 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("Error: Unable to read args")?;

	let seqs = FASTAVec::from_file(&filename);

	let muts = compare_strings(seqs[0].seq.as_slice(), seqs[1].seq.as_slice());
	let ratio = transition_transversion_ratio(&muts);

	println!("{:.13}", ratio);

	Ok(())
}
