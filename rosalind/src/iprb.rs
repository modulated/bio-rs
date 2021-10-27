use bio::*;

fn main() -> BioResult<()> {
	let res = combinatorics::mendelian::prob_inheritance_dominant(17, 30, 24);
	println!("{}", res);

	Ok(())
}
