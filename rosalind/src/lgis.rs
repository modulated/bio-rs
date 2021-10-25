use bio::{
	longest_decreasing_subsequence, longest_increasing_subsequence,
	util::{int_string_to_vec, slice_to_string},
};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("LGIS Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filestring = std::fs::read_to_string(args().nth(1).ok_or("File not found")?)?;
	let s = filestring.split_once('\n').unwrap().1.trim();
	let input = int_string_to_vec(s);

	let res1 = longest_increasing_subsequence(&input);
	println!("{}", slice_to_string(&res1));
	let res2 = longest_decreasing_subsequence(&input);
	println!("{}", slice_to_string(&res2));

	Ok(())
}
