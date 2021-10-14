use bio::combinatorics::potential_mrna_strings_from_protein as combo;
use bio::Seq;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("MRNA Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;

	let combos = combo(&Seq::new(input)[..], 1_000_000);

	println!("{}", combos);

	Ok(())
}
