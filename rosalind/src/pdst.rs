use bio::{hamming::matrix, Seq};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("PDST Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found")?;
	let fastas = bio::FASTAVec::from_file(&filename);

	let mut seqs: Vec<Seq> = vec![];
	for f in fastas {
		seqs.push(f.seq);
	}

	let res = matrix(&seqs);

	for l in res {
		for j in l {
			print!("{:.5} ", j);
		}
		println!();
	}

	Ok(())
}
