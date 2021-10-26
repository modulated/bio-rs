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

#[must_use]
pub fn int_string_to_vec(string: &str) -> Vec<u32> {
	string.split(' ').map(|x| x.parse().unwrap()).collect()
}
