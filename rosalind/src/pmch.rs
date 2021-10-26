use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("PMCH Problem");
	if args().len() != 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let file = args().nth(1).ok_or("File not found.")?;
	let fasta = bio::FASTA::from_file(&file);

	let out = bio::combinatorics::permutation::perfect_matchings(&fasta.seq);

	println!("{}", out);

	Ok(())
}
