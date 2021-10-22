use bio::{formats::parse_string_to_fasta_vec, Seq};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("SPLC Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;

	let fastas = parse_string_to_fasta_vec(&input);
	println!("{} sequences loaded", fastas.len());

	let introns: Vec<&Seq> = fastas[1..].iter().map(|x| &x.seq).collect();
	let splc = fastas[0].seq.splice_introns(&introns);

	println!("{}", splc.translate(true));

	Ok(())
}
