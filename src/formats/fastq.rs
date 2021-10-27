use crate::Seq;
use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct FASTQ {
	pub name: String,
	pub seq: Seq,
	pub qual: Seq,
}

impl FASTQ {
	#[must_use]
	pub fn new(string: &str) -> Self {
		let (name, data) = string
			.trim()
			.trim_start_matches('@')
			.split_once('\n')
			.unwrap();

		let (seq, qual) = data.trim().split_once('+').unwrap();

		Self {
			name: name.to_string(),
			seq: Seq::new(seq),
			qual: Seq::new(qual),
		}
	}

	#[must_use]
	pub fn from_file(file: &str) -> Self {
		Self::new(&std::fs::read_to_string(file).unwrap())
	}
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
			Seq::new(
				r"1:DAADDDF\<B\<AGF=FGIEHCCD9DG=1E9?D>CF@HHG??B\<GEBGHCG;;CDB8==C@@>>GII@@5?A?@B>CEDCFCC:;?CCCAC"
			)
		);
	}
}
