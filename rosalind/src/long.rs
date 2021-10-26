use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("LONG Problem");
	if args().len() != 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found")?;
	let fastas = bio::FASTAVec::from_file(&filename);
	let strings: Vec<Vec<u8>> = fastas.iter().map(|x| x.seq.as_slice().to_vec()).collect();

	let res = bio::shortest_superstring(strings);

	println!("{}", std::str::from_utf8(&res).unwrap());

	Ok(())
}
