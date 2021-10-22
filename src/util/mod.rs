mod fibonacci;
pub mod sets;
use std::fmt::Display;

pub use fibonacci::{mortal_rabbits, rabbits};

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
