use crate::{Seq, FASTQ};
use std::ops::{Index, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

#[derive(PartialEq, Debug)]
pub struct FASTQVec(Vec<FASTQ>);

impl FASTQVec {
	#[must_use]
	pub fn new(vec: Vec<FASTQ>) -> Self {
		Self(vec)
	}

	#[must_use]
	pub fn from_string(input: &str) -> Self {
		let mut res = vec![];
		let arr: Vec<&str> = input.split('\n').collect();
		for s in arr.chunks_exact(4) {
			let f = FASTQ {
				name: s[0][1..].to_string(),
				seq: Seq::new(s[1]),
				qual: Seq::new(s[3]),
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
	pub fn iter(&self) -> std::slice::Iter<FASTQ> {
		self.0.iter()
	}

	#[must_use]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.0.len() == 0
	}
}

impl Index<usize> for FASTQVec {
	type Output = FASTQ;

	fn index(&self, index: usize) -> &Self::Output {
		&self.0[index]
	}
}

impl Index<Range<usize>> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: Range<usize>) -> &[FASTQ] {
		&self.0[index]
	}
}

impl Index<RangeFull> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: RangeFull) -> &[FASTQ] {
		&self.0[index]
	}
}

impl Index<RangeFrom<usize>> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: RangeFrom<usize>) -> &[FASTQ] {
		&self.0[index]
	}
}

impl Index<RangeInclusive<usize>> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: RangeInclusive<usize>) -> &[FASTQ] {
		&self.0[index]
	}
}

impl Index<RangeTo<usize>> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: RangeTo<usize>) -> &[FASTQ] {
		&self.0[index]
	}
}

impl Index<RangeToInclusive<usize>> for FASTQVec {
	type Output = [FASTQ];

	fn index(&self, index: RangeToInclusive<usize>) -> &[FASTQ] {
		&self.0[index]
	}
}

impl IntoIterator for FASTQVec {
	type Item = FASTQ;

	type IntoIter = std::vec::IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.0.into_iter()
	}
}

pub struct Iter<'a> {
	vec: &'a FASTQVec,
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

impl<'a> IntoIterator for &'a FASTQVec {
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
	use crate::{FASTQVec, Seq, FASTQ};

	#[test]
	fn vec_from_string() {
		let input = r#"@SEQ_1
GCTCCCTGGGGGTGGTGGGAGTCGTAGTTCTCAGGAGATTCCCTCTTAAAACAAAGCAAAATGGGACGTTACGCCTCATTATTGCTGCTGATCGTTTAGGGTCTCGCTGCCCTGGACATTGCGATGCAATATCTAATCGGGTAGTGCCTCGCGAATATCCGCAGCGCGCCTCACCGAGGGACATTTAATTCACACTTAAA
+
GBDB@>DAGCJ?BI<C?IB?BADEIGJB9F;A=EF;GDHHDH@BH>A<;CDJCA@JE??DAGBCF?;>FDH;D:;DJ@=BCE@B7B@;C@DAJ?@EH@<FF6?BCI>IF@E?B?B8@>B@D@7:J@AA>4==?B:8D?<F><E<G=B97H=B<@AA;DH6>;@=1>A69B>;A>9I@;7@B698=4:;7=6537=78211
@SEQ_2
CGACGCTCTGCGGGTGCCCCCTCTCGTCTTTGTTATTATGACGGGCCAGCTGAGTCCAATAATCAGACCGGAAGCCCTATCATAGGGTTGATCCATATGAATTATATGATGGCGCAGACACATGAGGTAGACAACCCTTGCGCTAGCAGATTCCAGGTTCGTCTGACCGGCGCTCTAAAAATCGGTGCATGTGACCCGGA
+
9>AFHCD=GEJDH?CBB;;AD>;AC?H;;BCJBCAAAB?IACD>6>JGGB@>E>C>@=>;<C?D748E<E96;D<:BJ=8A:DDCC=A@A5C99=<AB??ABJ==I?5?@IA>FD?:D=:>>A7;:CAG9;8A9<=8A?8F>76<7?:?<BA8;BE>@@;B?89;7<959BH=;FC=.BH491E=55?;=?D8:)::0*(
@SEQ_3
GACGCCAATAAAGGATTTCTTACTGCATACAATTGCCTGCGCGCGAATTAGGGACCTGTCGTGAGTCTGTCGCGGGGCTGCCGTCGGTCTAGTGGGTAGACGCCCTATTTGGCGTCCGCCGTGTCTCACTAATCTAAACTGGGTTTGGGTGTAACGCGGGCGTGGACCTGGTAGCTTCTACGACTTCCGGGCTGTACTGG
+
=8@367B6=9=<8694<6B<976<-7:4:4A6988:<9<=:;A83;4:A8/6106<?;>B87:?449A<6726:<;7@;<80>45>3;+3:48=;8-578070:85:63-8::16/?:,67=492A69199@4.385320996;0656(3,17(.9=;05.348006/05024.2)1<1.6+**3940,11*.&(:1-2&"#;

		let vec = vec![
			FASTQ {
				name: "SEQ_1".to_string(),
				seq: Seq::new("GCTCCCTGGGGGTGGTGGGAGTCGTAGTTCTCAGGAGATTCCCTCTTAAAACAAAGCAAAATGGGACGTTACGCCTCATTATTGCTGCTGATCGTTTAGGGTCTCGCTGCCCTGGACATTGCGATGCAATATCTAATCGGGTAGTGCCTCGCGAATATCCGCAGCGCGCCTCACCGAGGGACATTTAATTCACACTTAAA"),				
				qual: Seq::new("GBDB@>DAGCJ?BI<C?IB?BADEIGJB9F;A=EF;GDHHDH@BH>A<;CDJCA@JE??DAGBCF?;>FDH;D:;DJ@=BCE@B7B@;C@DAJ?@EH@<FF6?BCI>IF@E?B?B8@>B@D@7:J@AA>4==?B:8D?<F><E<G=B97H=B<@AA;DH6>;@=1>A69B>;A>9I@;7@B698=4:;7=6537=78211")
			},
			FASTQ {
				name: "SEQ_2".to_string(),
				seq: Seq::new("CGACGCTCTGCGGGTGCCCCCTCTCGTCTTTGTTATTATGACGGGCCAGCTGAGTCCAATAATCAGACCGGAAGCCCTATCATAGGGTTGATCCATATGAATTATATGATGGCGCAGACACATGAGGTAGACAACCCTTGCGCTAGCAGATTCCAGGTTCGTCTGACCGGCGCTCTAAAAATCGGTGCATGTGACCCGGA"),
				qual:  Seq::new("9>AFHCD=GEJDH?CBB;;AD>;AC?H;;BCJBCAAAB?IACD>6>JGGB@>E>C>@=>;<C?D748E<E96;D<:BJ=8A:DDCC=A@A5C99=<AB??ABJ==I?5?@IA>FD?:D=:>>A7;:CAG9;8A9<=8A?8F>76<7?:?<BA8;BE>@@;B?89;7<959BH=;FC=.BH491E=55?;=?D8:)::0*(")
			},
			FASTQ {
				name: "SEQ_3".to_string(),
				seq: Seq::new("GACGCCAATAAAGGATTTCTTACTGCATACAATTGCCTGCGCGCGAATTAGGGACCTGTCGTGAGTCTGTCGCGGGGCTGCCGTCGGTCTAGTGGGTAGACGCCCTATTTGGCGTCCGCCGTGTCTCACTAATCTAAACTGGGTTTGGGTGTAACGCGGGCGTGGACCTGGTAGCTTCTACGACTTCCGGGCTGTACTGG"),
				qual: Seq::new("=8@367B6=9=<8694<6B<976<-7:4:4A6988:<9<=:;A83;4:A8/6106<?;>B87:?449A<6726:<;7@;<80>45>3;+3:48=;8-578070:85:63-8::16/?:,67=492A69199@4.385320996;0656(3,17(.9=;05.348006/05024.2)1<1.6+**3940,11*.&(:1-2&")
			}
		];
		let output = FASTQVec::new(vec);

		assert_eq!(output, FASTQVec::from_string(input));
	}
}