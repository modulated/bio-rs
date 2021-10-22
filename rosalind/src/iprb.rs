use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("IPRD Problem");
	// if args().len() < 2 {
	//     println!("Please supply file as argument.");
	//     return Ok(());
	// }

	// let filename = args().nth(1).ok_or("File not found.")?;
	// let input = std::fs::read_to_string(filename)?;
	// let v: Vec<u32> = input.split(' ').map(|s| s.parse().unwrap()).collect();

	let res = combinatorics::mendelian::prob_inheritance_dominant(17, 30, 24);
	println!("{}", res);

	Ok(())
}
