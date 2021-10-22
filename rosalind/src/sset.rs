use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("SSET Problem");
	if args().len() != 2 {
		println!("Please supply integer as argument.");
		return Ok(());
	}

	let n: u32 = args().skip(1).map(|x| x.parse().unwrap()).next().unwrap();

	let mut acc = 2;

	for _ in 1..n {
		acc = (acc * 2) % 1_000_000;
	}

	println!("{}", acc);

	Ok(())
}
