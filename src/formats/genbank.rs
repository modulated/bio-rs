pub struct GenBank {
	pub name: String,
	pub len: usize,
}

impl GenBank {
	#[must_use]
	pub fn new(file: &str) -> Self {
		let data = std::fs::File::open(file).unwrap();
		let gb = gb_io::reader::SeqReader::new(&data)
			.next()
			.unwrap()
			.unwrap();

		Self {
			name: gb.name.unwrap(),
			len: gb.len.unwrap(),
		}
	}
}

#[cfg(test)]
mod test {
	use crate::GenBank;

	#[test]
	fn genbank() {
		let gb = GenBank::new("benches/scu49845.gb");
		assert_eq!(gb.name, "SCU49845");
		assert_eq!(gb.len, 5028);
	}
}
