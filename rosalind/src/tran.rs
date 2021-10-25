use bio::{compare_strings, formats::parse_string_to_fasta_vec, transition_transversion_ratio};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("TRAN Problem");
	if args().len() != 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("Error: Unable to read args")?;
	let input = std::fs::read_to_string(filename)?;

	let seqs = parse_string_to_fasta_vec(&input);

	let muts = compare_strings(seqs[0].seq.as_slice(), seqs[1].seq.as_slice());
	let ratio = transition_transversion_ratio(&muts);

	println!("{:.13}", ratio);

	Ok(())
}
