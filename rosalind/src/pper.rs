use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("PPER Problem");
	if args().len() < 2 {
		println!("Please supply num as argument.");
		return Ok(());
	}

	let intg = args()
		.skip(1)
		.map(|x| x.parse().unwrap())
		.collect::<Vec<_>>();

	let out =
		bio::combinatorics::permutation::partial_permutation_modulo(intg[0], intg[1], 1_000_000);

	println!("{}", out);

	Ok(())
}
