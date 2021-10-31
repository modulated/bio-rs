use crate::{BioError, BioResult};
use std::fmt::Display;

pub fn slice_to_string<T: Display>(slice: &[T]) -> String {
	slice
		.iter()
		.map(std::string::ToString::to_string)
		.collect::<Vec<_>>()
		.join(" ")
}

pub fn slice_to_fmt_array<T: Display>(slice: &[T]) -> String {
	let r = slice
		.iter()
		.map(std::string::ToString::to_string)
		.collect::<Vec<_>>()
		.join(", ");
	format!("{{{}}}", r)
}

/// Takes a string reference and returns `Vec<u32`.
/// # Errors
/// Will return `BioError::ParseIntError` if unable to parse input as int.
pub fn int_string_to_vec(string: &str) -> BioResult<Vec<u32>> {
	string
		.split(' ')
		.map(str::parse)
		.collect::<Result<Vec<_>, _>>()
		.map_err(BioError::from)
}
