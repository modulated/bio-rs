use bio::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("FIB Problem");

	let mut memo = std::collections::HashMap::new();
	let res = util::rabbits(36, 3, &mut memo);
	println!("{}", res);

	Ok(())
}
