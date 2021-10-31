pub struct GenBank(pub gb_io::seq::Seq);

impl GenBank {
	#[must_use]
	pub fn new(file: &str) -> Self {
		let data = std::fs::File::open(file).unwrap();
		let gb = gb_io::reader::SeqReader::new(&data)
			.next()
			.unwrap()
			.unwrap();

		Self(gb)
	}
}

#[cfg(test)]
mod test {
	use crate::GenBank;

	#[test]
	fn genbank() {
		let gb = GenBank::new("benches/scu49845.gb");
		assert_eq!(gb.0.name.unwrap(), "SCU49845");
		assert_eq!(gb.0.len.unwrap(), 5028);
        assert_eq!(gb.0.molecule_type.unwrap(), "DNA");
        assert_eq!(gb.0.date.unwrap().to_string(), "21-JUN-1999");
        assert_eq!(gb.0.seq[0], b'g');
        assert_eq!(gb.0.seq[4981], b'g');
	}
}
