use bio::{hamming::distance, Seq};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("HAMM Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}
	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;

	let parsed: Vec<String> = input
		.split('\n')
		.map(|x| x.split_whitespace().collect())
		.collect();

	let a = Seq::new(&parsed[0]);
	let b = Seq::new(&parsed[1]);

	let h = distance(&a.iter(), &b.iter());

	println!("{}", h);

	Ok(())
}
