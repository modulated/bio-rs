use crate::{BioResult, Seq};
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct FASTQ {
	pub name: String,
	pub seq: Seq,
	pub qual: Vec<u8>,
}

impl FASTQ {
	#[must_use]
	pub fn new(string: &str) -> Self {
		let string: Vec<&str> = string.split('\n').collect();

		let out = Self {
			name: string[0].trim_start_matches('@').to_string(),
			seq: Seq::new(string[1].trim()),
			qual: string[3].trim().as_bytes().to_vec(),
		};

		out
	}

	#[must_use]
	pub fn from_file(file: &str) -> Self {
		Self::new(&std::fs::read_to_string(file).unwrap())
	}

	#[must_use]
	pub fn phred_quality(&self) -> Vec<u8> {
		self.qual.iter().map(|x| phred_to_int(*x)).collect()
	}

	pub fn average_quality(&self) -> BioResult<f64> {
		use std::convert::TryFrom;
		let num: f64 = self
			.phred_quality()
			.iter()
			.map(|x| u32::from(*x))
			.sum::<u32>()
			.into();
		let denom: f64 = u32::try_from(self.qual.len())?.into();
		Ok(num / denom)
	}
}

const fn phred_to_int(ph: u8) -> u8 {
	ph - b'!'
}

impl Display for FASTQ {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let seq = self
			.seq
			.0
			.chunks(70)
			.map(|x| String::from_utf8(x.to_vec()).unwrap())
			.collect::<Vec<_>>()
			.join("\n");
		write!(f, "{}\n{}", self.name, seq)
	}
}

#[cfg(test)]
mod test {
	use crate::{Seq, FASTQ};

	#[test]
	fn new() {
		let f = FASTQ::new(
			r"@HWI-ST999:102:D1N6AACXX:1:1101:1235:1936 1:N:0:
			ATGTCTCCTGGACCCCTCTGTGCCCAAGCTCCTCATGCATCCTCCTCAGCAACTTGTCCTGTAGCTGAGGCTCACTGACTACCAGCTGCAG
			+
			1:DAADDDF\<B\<AGF=FGIEHCCD9DG=1E9?D>CF@HHG??B\<GEBGHCG;;CDB8==C@@>>GII@@5?A?@B>CEDCFCC:;?CCCAC
			",
		);
		assert_eq!(f.name, "HWI-ST999:102:D1N6AACXX:1:1101:1235:1936 1:N:0:");
		assert_eq!(f.seq, Seq::new("ATGTCTCCTGGACCCCTCTGTGCCCAAGCTCCTCATGCATCCTCCTCAGCAACTTGTCCTGTAGCTGAGGCTCACTGACTACCAGCTGCAG"));
		assert_eq!(
			f.qual,
			br#"1:DAADDDF\<B\<AGF=FGIEHCCD9DG=1E9?D>CF@HHG??B\<GEBGHCG;;CDB8==C@@>>GII@@5?A?@B>CEDCFCC:;?CCCAC"#
		);
	}

	#[test]
	fn phred_quality() {
		let f = FASTQ::new(
			r#"@Rosalind_0041
			GGCCGGTCTATTTACGTTCTCACCCGACGTGACGTACGGTCC
			+
			6.3536354;.151<211/0?::6/-2051)-*"40/.,+%))"#,
		);
		let output = vec![
			21, 13, 18, 20, 18, 21, 18, 20, 19, 26, 13, 16, 20, 16, 27, 17, 16, 16, 14, 15, 30, 25,
			25, 21, 14, 12, 17, 15, 20, 16, 8, 12, 9, 1, 19, 15, 14, 13, 11, 10, 4, 8, 8,
		];

		assert_eq!(f.phred_quality(), output);
	}

	#[test]
	fn average_quality() {
		let f = FASTQ::new(
			r#"@Rosalind_0041
			GGCCGGTCTATTTACGTTCTCACCCGACGTGACGTACGGTCC
			+
			6.3536354;.151<211/0?::6/-2051)-*"40/.,+%))"#,
		);
		assert_eq!(
			format!("{:.4}", f.average_quality().unwrap()),
			16.0698.to_string()
		);
	}

	#[test]
	fn phred_convert() {
		use super::phred_to_int as phr;
		assert_eq!(phr(b'!'), 0);
		assert_eq!(phr(b'A'), 32);
	}
}
