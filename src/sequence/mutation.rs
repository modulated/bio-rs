#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Mutation {
	None,
	Transition,
	Transversion,
}

fn compare_bases(left: u8, right: u8) -> Mutation {
	if left == right {
		return Mutation::None;
	}

	match (left.to_ascii_uppercase(), right.to_ascii_uppercase()) {
		(b'A' | b'G', b'C' | b'T') | (b'C' | b'T', b'A' | b'G') => Mutation::Transversion,
		(b'G', b'A') | (b'A', b'G') | (b'T', b'C') | (b'C', b'T') => Mutation::Transition,
		_ => unreachable!("Cannot compare non-nucleotide chars"),
	}
}

#[must_use]
pub fn compare_strings(left: &[u8], right: &[u8]) -> Vec<Mutation> {
	left.iter()
		.zip(right.iter())
		.map(|x| compare_bases(*x.0, *x.1))
		.collect()
}

#[must_use]
pub fn transition_transversion_ratio(muts: &[Mutation]) -> f64 {
	let out = muts.iter().fold((0, 0), |a, x| match x {
		Mutation::Transition => (a.0 + 1, a.1),
		Mutation::Transversion => (a.0, a.1 + 1),
		Mutation::None => a,
	});
	f64::from(out.0) / f64::from(out.1)
}

#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_compare_bases() {
		assert_eq!(compare_bases(b'T', b'C'), Mutation::Transition);
		assert_eq!(compare_bases(b'A', b'A'), Mutation::None);
		assert_eq!(compare_bases(b'A', b'C'), Mutation::Transversion);
	}

	#[test]
	fn test_compare_strings() {
		let a = b"ACGGTG";
		let b = b"ATCACG";
		let out = vec![
			Mutation::None,
			Mutation::Transition,
			Mutation::Transversion,
			Mutation::Transition,
			Mutation::Transition,
			Mutation::None,
		];

		assert_eq!(compare_strings(a, b), out);
	}

	#[test]
	fn test_trans_trans_ratio() {
		let a =
			b"GCAACGCACAACGAAAACCCTTAGGGACTGGATTATTTCGTGATCGTTGTAGTTATTGGAAGTACGGGCATCAACCCAGTT";
		let b =
			b"TTATCTGACAAAGAAAGCCGTCAACGGCTGGATAATTTCGCGATCGTGCTGGTTACTGGCGGTACGAGTGTTCCTTTGGGT";
		let out = "1.21428571428";

		let muts = compare_strings(a, b);
		let r = transition_transversion_ratio(&muts);

		assert_eq!(out, format!("{:.13}", r.to_string()));
	}
}
