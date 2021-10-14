use bio::fasta::parse_string_to_vec_of_fasta;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("ORF Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;
	let v = parse_string_to_vec_of_fasta(&input);

	let orfs = v[0].seq.orf();
	for o in orfs {
		println!("{}", o);
	}

	Ok(())
}
