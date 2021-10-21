use super::bytes;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Seq(pub(crate) Vec<u8>);

impl Seq {
	pub fn new<T: Into<String>>(string: T) -> Self {
		let mut s: Vec<u8> = string.into().into();
		s.retain(|x| x.is_ascii_alphabetic());
		Seq(s)
	}

	pub fn from_bytes(b: &[u8]) -> Self {
		Seq(b.to_vec())
	}

	pub fn to_slice(&self) -> &[u8] {
		&self.0[..]
	}

	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn len(&self) -> usize {
		self.0.len()
	}

	pub fn iter(&self) -> std::slice::Iter<u8> {
		self.0.iter()
	}

	pub fn counts(&self) -> (u32, u32, u32, u32) {
		let mut out = (0, 0, 0, 0);
		for c in self.0.iter() {
			match c {
				b'A' | b'a' => out.0 += 1,
				b'C' | b'c' => out.1 += 1,
				b'G' | b'g' => out.2 += 1,
				b'T' | b't' | b'U' | b'u' => out.3 += 1,
				_ => panic!("Unexpected input in Seq: {}", c),
			}
		}

		out
	}

	pub fn complement(&self) -> Self {
		Self::new(String::from_utf8(bytes::complement_slice(&self.0)).unwrap())
	}

	pub fn reverse_complement(&self) -> Seq {
		let mut out = bytes::complement_slice(&self.0);
		out.reverse();
		Self::new(String::from_utf8(out).unwrap())
	}

	pub fn gc_content(&self) -> f64 {
		use std::convert::TryFrom;

		let mut count = 0;
		for c in self.iter() {
			match c {
				b'G' | b'g' | b'C' | b'c' => count += 1,
				_ => continue,
			}
		}

		f64::from(100 * count) / f64::from(i32::try_from(self.len()).unwrap())
	}

	pub fn transcribe(&self) -> Self {
		let mut out: Vec<u8> = Vec::with_capacity(self.0.len());
		for c in self.0.iter() {
			match c {
				b'U' => out.push(b'T'),
				b'u' => out.push(b't'),
				b'T' => out.push(b'U'),
				b't' => out.push(b'u'),
				_ => out.push(*c),
			}
		}
		Self::from_bytes(&out)
	}

	pub fn translate(&self, terminates: bool) -> Self {
		let mut out: Vec<u8> = Vec::with_capacity(self.0.len());
		for c in self
			.0
			.chunks_exact(3)
			.map(|x| super::bytes::codon_to_amino(x))
		{
			if terminates && c == b'*' {
				break;
			}
			out.push(c);
		}
		Self::from_bytes(&out)
	}

	pub fn suffix_overlap(&self, seq: &Self, len: usize) -> bool {
		if self.len() < len {
			return false;
		}

		let tail = &self.0[self.0.len() - len..];

		for (i, e) in tail.iter().enumerate().take(len) {
			if e != &seq.0[i] {
				return false;
			}
		}

		true
	}

	pub fn prefix_overlap(&self, seq: &Self, len: usize) -> bool {
		if seq.len() < len {
			return false;
		}

		let tail = &seq.0[seq.0.len() - len..];

		for (i, e) in tail.iter().enumerate().take(len) {
			if e != &self.0[i] {
				return false;
			}
		}

		true
	}

	pub fn substring(&self, pattern: &Seq) -> Vec<usize> {
		crate::alignment::substring(&pattern.0, &self.0)
	}

	pub fn splice_introns(&self, introns: &[&Seq]) -> Self {
		let mut arr = self.clone();

		for s in introns {
			let subidx = arr.substring(s);
			for si in subidx {
				let start_slice = si - 1;
				let end_slice = start_slice + s.len();
				arr.0.drain(start_slice..end_slice);
			}
		}

		arr
	}

	pub fn orf(&self) -> Vec<Seq> {
		use super::bytes::codon_to_amino;

		let mut out = vec![];

		let offset_1 = &self.0[1..];
		let offset_2 = &self.0[2..];
		let mut rev = bytes::complement_slice(&self.0);
		rev.reverse();
		let r_offset_1 = &rev[1..];
		let r_offset_2 = &rev[2..];

		let seqs = vec![
			&self.0[..],
			offset_1,
			offset_2,
			&rev[..],
			r_offset_1,
			r_offset_2,
		];

		for s in seqs {
			let mut proteins: Vec<Vec<u8>> = vec![];
			for c in s.chunks_exact(3) {
				match codon_to_amino(c) {
					b'M' => {
						proteins.iter_mut().for_each(|x| x.push(b'M'));
						proteins.push(vec![b'M'])
					}
					b'*' => {
						out.append(&mut proteins);
					}
					_c => {
						proteins.iter_mut().for_each(|x| x.push(_c));
					}
				}
			}
		}

		let mut out_seqs: Vec<Seq> = out.iter().map(|x| Seq::from_bytes(&x[..])).collect();
		out_seqs.sort();
		out_seqs.dedup();
		out_seqs
	}
}

impl Default for Seq {
	fn default() -> Self {
		Self(Default::default())
	}
}

impl<Idx> std::ops::Index<Idx> for Seq
where
	Idx: std::slice::SliceIndex<[u8]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.0[index]
	}
}

impl std::fmt::Display for Seq {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", String::from_utf8(self.0.clone()).unwrap())
	}
}

#[cfg(test)]
mod test {
	use crate::formats::parse_string_to_vec_of_fasta;

	use super::Seq;

	#[test]
	fn new_seq() {
		let input = "ACTCGTAGCTAGCTAGC";
		assert_eq!(Seq::new(input).0, input.as_bytes());

		let input_2 = "MKNKFKTQEELVNHLKTVGFVFANSEIYNGLANAWDYGPLGVLLKNNLKNLWWKEFVTKQKDVVGLDSAIILNPLVWKASGHLDNFSDPLIDCKNCKARYRADKLIESFDENIHIAENSSNEEFAKVLNDYEISCPTCKQFNWTEIRHFNLMFKTYQGVIEDAKNVVYLRPETAQGIFVNFKNVQRSMRLHLPFGIAQIGKSFRNEITPGNFIFRTREFEQMEIEFFLKEESAYDIFDKYLNQIENWLVSACGLSLNNLRKHEHPKEELSHYSKKTIDFEYNFLHGFSELYGIAYRTNYDLSVHMNLSKKDLTYFDEQTKEKYVPHVIEPSVGVERLLYAILTEATFIEKLENDDERILMDLKYDLAPYKIAVMPLVNKLKDKAEEIYGKILDLNISATFDNSGSIGKRYRRQDAIGTIYCLTIDFDSLDDQQDPSFTIRERNSMAQKRIKLSELPLYLNQKAHEDFQRQCQK";
		assert_eq!(Seq::new(input_2).to_string(), input_2)
	}

	#[test]
	fn default() {
		assert_eq!(Seq::default(), Seq::new(""));
	}

	#[test]
	fn to_string() {
		let input = "ACTCGTAGCTAGCTAGC";
		assert_eq!(Seq::new(input).to_string(), input);
	}

	#[test]
	fn len() {
		let input = "ACTCGTAGCTAGCTAGC";
		assert_eq!(Seq::new(input).len(), 17);
	}

	#[test]
	fn counts() {
		let input = "ACTCGTAGCTAGCTAGC";
		assert_eq!(Seq::new(input).counts(), (4, 5, 4, 4))
	}

	#[test]
	fn orf() {
		let input = "AGCCATGTAGCTAACTCAGGTTACATGGGGATGACCCCGCGACTTGGATTAGAGTCTCTTTTGGAATAAGCCTGAATGATCCGAGTAGCATCTCAG";
		let mut output = vec![
			"MLLGSFRLIPKETLIQVAGSSPCNLS",
			"M",
			"MGMTPRLGLESLLE",
			"MTPRLGLESLLE",
		];

		let mut stringvec = Seq::new(input)
			.orf()
			.iter()
			.map(|x| x.to_string())
			.collect::<Vec<_>>();
		stringvec.sort();
		output.sort();

		assert_eq!(stringvec, output);
	}

	#[test]
	fn transcribe() {
		let input = "AGCATCAGTG";
		let output = "AGCAUCAGUG";

		assert_eq!(output, Seq::new(input).transcribe().to_string());
	}

	#[test]
	fn translate() {
		let input = "ATGGCCATGGCGCCCAGAACTGAGATCAATAGTACCCGTATTAACGGGTGA";
		let output1 = "MAMAPRTEINSTRING";
		let output2 = "MAMAPRTEINSTRING*";
		let res1 = Seq::new(input).translate(true).to_string();
		let res2 = Seq::new(input).translate(false).to_string();
		assert_eq!(res1, output1);
		assert_eq!(res2, output2);
	}

	#[test]
	fn reverse_complement() {
		let input = "ACTGAC";
		let output = "GTCAGT";

		assert_eq!(output, Seq::new(input).reverse_complement().to_string());
	}

	#[test]
	fn gc_content() {
		let input = "CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT";
		let output = "60.919540";

		assert_eq!(output, &Seq::new(input).gc_content().to_string()[..9]);
	}

	#[test]
	fn suffix_overlap() {
		let input = vec!["AAATTTGGG", "GGGGTTACCC"];
		let s1 = Seq::new(input[0]);
		let s2 = Seq::new(input[1]);

		assert_eq!(true, s1.suffix_overlap(&s2, 3));
		assert_eq!(false, s2.suffix_overlap(&s1, 3));
	}

	#[test]
	fn prefix_overlap() {
		let input = vec!["AAATTTGGG", "GGGGTTACCC"];
		let s1 = Seq::new(input[0]);
		let s2 = Seq::new(input[1]);

		assert_eq!(false, s1.prefix_overlap(&s2, 3));
		assert_eq!(true, s2.prefix_overlap(&s1, 3));
	}

	#[test]
	fn substring() {
		let a = Seq::new("GATATATGCATATACTT");
		let b = Seq::new("ATAT");

		let res = a.substring(&b);

		assert_eq!(vec![2, 4, 10], res);
	}

	#[test]
	fn splice() {
		let input = ">Rosalind_10
		ATGGTCTACATAGCTGACAAACAGCACGTAGCAATCGGTCGAATCTCGAGAGGCATATGGTCACATGATCGGTCGAGCGTGTTTCAAAGTTTGCGCCTAG
		>Rosalind_12
		ATCGGTCGAA
		>Rosalind_15
		ATCGGTCGAGCGTGT";
		let dnaout = "ATGGTCTACATAGCTGACAAACAGCACGTAGCATCTCGAGAGGCATATGGTCACATGTTCAAAGTTTGCGCCTAG";
		let protout = "MVYIADKQHVASREAYGHMFKVCA";
		let fastas = parse_string_to_vec_of_fasta(input);
		let introns: Vec<&Seq> = fastas[1..].iter().map(|x| &x.seq).collect();
		let res = fastas[0].seq.splice_introns(&introns[..]);

		assert_eq!(res.to_string(), dnaout);
		assert_eq!(res.translate(true).to_string(), protout);
	}
}
