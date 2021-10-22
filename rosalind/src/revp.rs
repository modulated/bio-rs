use bio::{formats::parse_string_to_fasta_vec, palindrome};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("REVP Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("Error: Unable to read args")?;
	let input = std::fs::read_to_string(filename)?;

	let seq = &parse_string_to_fasta_vec(&input)[0].seq;

	let res = palindrome::reverse(seq, 4, 12);

	let mut s = "".to_string();
	for (i, j) in res {
		use std::fmt::Write;
		writeln!(s, "{} {}", i, j)?;
	}
	println!("{}", s);

	Ok(())
}
