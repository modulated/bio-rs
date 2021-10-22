use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("SCSP Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found")?;
	let input = std::fs::read_to_string(filename)?;
	let _ = input.trim();

	let mut split = input.split('\n');
	let a = split.next().unwrap();
	let b = split.next().unwrap();

	let res: String = String::from_utf8(bio::shortest_common_supersequence(
		a.as_bytes(),
		b.as_bytes(),
	))
	.unwrap();

	println!("{}", res);

	Ok(())
}
