use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("LEXF Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filestring = std::fs::read_to_string(args().nth(1).ok_or("File not found")?)?;
	let s: Vec<_> = filestring.split('\n').collect();
	let alpha: Vec<char> = s[0].split(' ').map(|x| x.chars().next().unwrap()).collect();
	let n: usize = s[1].parse()?;

	let out = bio::combinatorics::kmer::ordered_kmer_permutations(&alpha, n);

	for x in out {
		println!("{}", x.iter().collect::<String>());
	}

	Ok(())
}
