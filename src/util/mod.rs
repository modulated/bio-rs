mod fibonacci;
pub mod sets;
use std::fmt::Display;

pub use fibonacci::{fibonacci_rabbits, mortal_fibonacci_rabbits};

pub fn slice_to_string<T: Display>(slice: &[T]) -> String {
	slice
		.iter()
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
		.join(" ")
}

pub fn slice_to_fmt_array<T: Display>(slice: &[T]) -> String {
	let r = slice
		.iter()
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
		.join(", ");
	format!("{{{}}}", r)
}
