use bio::{overlap::create_graph, FASTAVec};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("GRPH Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	
	let fastas = FASTAVec::from_file(&filename);
	println!("{} sequences loaded", fastas.len());

	let edges = crate::create_graph(&fastas, 3);

	for (k, v) in &edges {
		println!("{} {}", k, v);
	}

	println!("{} edges found", &edges.len());

	Ok(())
}
