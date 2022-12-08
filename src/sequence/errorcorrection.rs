use std::collections::HashMap;

use crate::Seq;

#[must_use]
pub fn error_correction(seqs: &[Seq]) -> Vec<(Seq, Seq)> {
	let mut counts: HashMap<Vec<u8>, usize> = HashMap::new();
	let out = Vec::new();

	for i in seqs {
		let mut e = counts.entry(i.0.clone()).or_default();
		*e += 1;

		e = counts.entry(i.reverse_complement().0).or_default();
		*e += 1;
	}

	let unique: Vec<Vec<u8>> = counts
		.iter()
		.filter(|(_, v)| **v != 1)
		.map(|(k, _)| k)
		.cloned()
		.collect();
	for x in unique {
		println!("{}", std::str::from_utf8(&x).unwrap());
	}
	out
}

#[cfg(test)]
mod test {
	use crate::Seq;

	#[test]
	fn correct() {
		let input = [
			Seq::new("TCATC"),
			Seq::new("TTCAT"),
			Seq::new("TCATC"),
			Seq::new("TGAAA"),
			Seq::new("GAGGA"),
			Seq::new("TTTCA"),
			Seq::new("ATCAA"),
			Seq::new("TTGAT"),
			Seq::new("TTTCC"),
		];
		let output = [
			[Seq::new("TTCAT"), Seq::new("TTGAT")],
			[Seq::new("GAGGA"), Seq::new("GATGA")],
			[Seq::new("TTTCC"), Seq::new("TTTCA")],
		];

		let res = super::error_correction(&input);

		for (i, j) in res.iter().zip(output.iter()) {
			assert_eq!(i.0, j[0]);
			assert_eq!(i.1, j[1]);
		}
	}
}
