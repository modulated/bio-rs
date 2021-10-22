use bio::util::{sets, slice_to_fmt_array, slice_to_string};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	println!("RNA Problem");
	if args().len() < 2 {
		println!("Please supply file as argument.");
		return Ok(());
	}

	let filename = args().nth(1).ok_or("A")?;
	let input = std::fs::read_to_string(filename)?;

	let (n, x) = input.trim().split_once('\n').unwrap();
	let (l, r) = x.split_once('\n').unwrap();
	let left: Vec<usize> = l
		.trim_start_matches('{')
		.trim_end_matches('}')
		.split(", ")
		.map(|x| x.parse().unwrap())
		.collect();
	let right: Vec<usize> = r
		.trim_start_matches('{')
		.trim_end_matches('}')
		.split(", ")
		.map(|x| x.parse().unwrap())
		.collect();

	let n_seq = (1..=n.parse().unwrap()).collect::<Vec<_>>();

	println!("{}", slice_to_fmt_array(&sets::union(&left, &right)));
	println!("{}", slice_to_fmt_array(&sets::intersection(&left, &right)));
	println!("{}", slice_to_fmt_array(&sets::difference(&left, &right)));
	println!("{}", slice_to_fmt_array(&sets::difference(&right, &left)));
	println!("{}", slice_to_fmt_array(&sets::difference(&n_seq, &left)));
	println!("{}", slice_to_fmt_array(&sets::difference(&n_seq, &right)));

	Ok(())
}
