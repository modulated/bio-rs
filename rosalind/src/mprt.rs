use bio::{formats::FASTA, protein_motif::ProteinMotif};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("MPRT Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found.")?;
	let input = std::fs::read_to_string(filename)?;

	let ids: Vec<_> = input.trim_end().split('\n').collect();

	let pm = ProteinMotif::new("N{P}[ST]{P}");

	for id in ids {
		let f = FASTA::from_uniprot_id(id);
		let res = pm.find_in(&f.seq);

		if !res.is_empty() {
			let str_ints = res
				.iter()
				.map(std::string::ToString::to_string)
				.collect::<Vec<_>>()
				.join(" ");
			println!("{}", id);
			println!("{}", str_ints);
		}
	}

	Ok(())
}
