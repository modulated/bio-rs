fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut args = std::env::args();
	if args.len() != 2 {
		panic!("Need file as argument");
	}
	let seq = bio::Seq::from_file(&args.nth(1).unwrap()).counts();
	println!("{} {} {} {}", seq.0, seq.1, seq.2, seq.3);
	Ok(())
}
