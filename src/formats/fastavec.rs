use crate::{Seq, FASTA};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(PartialEq, Debug)]
pub struct FASTAVec(Vec<FASTA>);

impl FASTAVec {
	#[must_use]
	pub fn new(vec: Vec<FASTA>) -> Self {
		Self(vec)
	}

	#[must_use]
	pub fn from_string(input: &str) -> Self {
		let mut res = vec![];
		let arr: Vec<&str> = input.split('>').skip(1).collect();

		for s in arr {
			let v: Vec<String> = s
				.splitn(2, '\n')
				.map(|n| n.split_whitespace().collect::<Vec<&str>>().join(""))
				.collect();

			let f = FASTA {
				name: v[0].clone(),
				seq: Seq::new(&v[1]),
			};
			res.push(f);
		}

		Self(res)
	}

	#[must_use]
	pub fn from_file(file: &str) -> Self {
		let s = std::fs::read_to_string(file).unwrap();
		Self::from_string(&s)
	}

	#[must_use]
	pub fn iter(&self) -> std::slice::Iter<FASTA> {
		self.0.iter()
	}
}

impl Index<usize> for FASTAVec {
	type Output = FASTA;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl Index<Range<usize>> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: Range<usize>) -> &[FASTA] {
		&self.0[index]
	}
}

impl Index<RangeFull> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: RangeFull) -> &[FASTA] {
		&self.0[index]
	}
}

impl Index<RangeFrom<usize>> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: RangeFrom<usize>) -> &[FASTA] {
		&self.0[index]
	}
}

impl Index<RangeInclusive<usize>> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: RangeInclusive<usize>) -> &[FASTA] {
		&self.0[index]
	}
}

impl Index<RangeTo<usize>> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: RangeTo<usize>) -> &[FASTA] {
		&self.0[index]
	}
}

impl Index<RangeToInclusive<usize>> for FASTAVec {
	type Output = [FASTA];

	fn index(&self, index: RangeToInclusive<usize>) -> &[FASTA] {
		&self.0[index]
	}
}

impl IntoIterator for FASTAVec {
    type Item = FASTA;

    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

pub struct Iter<'a> {
	vec: &'a FASTAVec,
	index: usize,
}

impl<'a> Iterator for Iter<'a> {
	type Item = &'a [u8];
	fn next(&mut self) -> Option<&'a [u8]> {
		let result = self.vec.0[self.index].seq.as_slice();
		self.index += 1;
		Some(result)
	}
}

impl<'a> IntoIterator for &'a FASTAVec {
	type Item = &'a [u8];

	type IntoIter = Iter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		Iter {
			vec: self,
			index: 0,
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{FASTAVec, Seq, FASTA};

	#[test]
	fn vec_from_string() {
		let input = r#">Rosalind_6404
        CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
        TCCCACTAATAATTCTGAGG
        >Rosalind_5959
        CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
        ATATCCATTTGTCAGCAGACACGC
        >Rosalind_0808
        CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
        TGGGAACCTGCGGGCAGTAGGTGGAAT"#;

		let vec = vec![
			FASTA {
				name: "Rosalind_6404".to_string(),
				seq: Seq::new(
					"CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAG
                GCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG",
				),
			},
			FASTA {
				name: "Rosalind_5959".to_string(),
				seq: Seq::new(
					"CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCG
                CTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC",
				),
			},
			FASTA {
				name: "Rosalind_0808".to_string(),
				seq: Seq::new(
					"CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTC
                AGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT",
				),
			},
		];
		let output = FASTAVec::new(vec);

		assert_eq!(output, FASTAVec::from_string(input));
	}
}
