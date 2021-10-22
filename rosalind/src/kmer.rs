use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("KMER Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filestring = std::fs::read_to_string(args().nth(1).ok_or("File not found")?)?;
	let fasta = &bio::formats::parse_string_to_fasta_vec(&filestring)[0];

	let r = bio::combinatorics::kmer::lexographical_kmer_composition(
		fasta.seq.as_slice(),
		&[b'A', b'C', b'G', b'T'][..],
		4,
	);

	for x in r {
		print!("{} ", x);
	}
	println!();

	Ok(())
}
