fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut args = std::env::args();
	if args.len() != 2 {
		panic!("Need file as argument");
	}
	let f = args.nth(1).ok_or("Unable to open file")?;
	let txt = std::fs::read_to_string(&f)?;

	let splt: Vec<&str> = txt.trim().split(' ').map(|x| x.trim()).collect();

	let mut vec = vec![];
	println!("{} IDs found", splt.len());
	for i in splt {
		println!("Getting {}", i);
		vec.push(bio::FASTA::from_ena_id(i));
	}

	let mut min = usize::MAX;
	let mut idx = 0;
	for (i, v) in vec.iter().enumerate() {
		let len = v.seq.len();
		println!("{} {}", i, len);
		if len < min {
			idx = i;
			min = len;
		}
	}

	println!("{}", vec[idx]);

	Ok(())
}
