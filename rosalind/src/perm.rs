use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("PERM Problem");
	if args().len() < 2 {
		println!("Please supply num as argument.");
		return Ok(());
	}

	let intg = args().nth(1).ok_or("File not found.")?;
	let input: u8 = intg.parse()?;

	let out = bio::combinatorics::permutation::permutation(input);

	println!("{}", out.len());
	for l in out {
		println!(
			"{}",
			l.iter()
				.map(std::string::ToString::to_string)
				.collect::<Vec<_>>()
				.join(" ")
		);
	}

	Ok(())
}
