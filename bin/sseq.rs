use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("SSEQ Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found")?;
	let input = std::fs::read_to_string(filename)?;

	let fastas = bio::formats::parse_string_to_vec_of_fasta(&input);
	let hay = &fastas[0].seq;
	let nd = &fastas[1].seq;

	let res = bio::subsequence(&nd[..], &hay[..])
		.iter()
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
		.join(" ");

	println!("{}", res);

	Ok(())
}
