use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("LCSQ Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("File not found")?;	
	let fastas = bio::FASTAVec::from_file(&filename);
	let a = &fastas[0].seq;
	let b = &fastas[1].seq;

	let res = bio::longest_common_subsequence(a.as_slice(), b.as_slice());
	let res_string = String::from_utf8(res).unwrap();

	println!("{}", res_string);

	Ok(())
}
