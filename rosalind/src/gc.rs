use bio::FASTAVec;
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("GC Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;

	let fastas = FASTAVec::from_file(&filename);
	let mut map = std::collections::HashMap::new();

	for f in fastas {
		map.insert(f.name, f.seq.gc_content());
	}

	let max = map
		.iter()
		.max_by(|a, b| (*a.1).partial_cmp(b.1).unwrap())
		.unwrap();
	println!("{} {}", *max.0, *max.1);

	Ok(())
}
