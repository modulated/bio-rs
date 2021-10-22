use bio::combinatorics::mendelian::prob_heterozygous_child as het;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("LIA Problem");

	let args: Vec<_> = std::env::args().skip(1).take(2).collect();
	if args.len() != 2 {
		panic!("Please supply 2 integers as args");
	}

	let (k, n) = (args[0].parse().unwrap(), args[1].parse().unwrap());

	let res = het(k, n);
	println!("{}", res);

	Ok(())
}
