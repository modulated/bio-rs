use crate::{contains, suffix_overlap};

#[must_use]
pub fn shortest_superstring(mut strings: Vec<Vec<u8>>) -> Vec<u8> {
	let mut res = strings[0].clone();

	while !strings.is_empty() {
		strings.retain(|seq| {
			if contains(seq, &res) {
				return false;
			}
			let mut found = false;
			for len in ((seq.len() / 2 - 1)..seq.len()).rev() {
				if suffix_overlap(&res, seq, len) {
					res.extend_from_slice(&seq[len..]);
					found = true;
					break;
				} else if suffix_overlap(seq, &res, len) {
					let mut tmp = seq.clone();
					tmp.extend_from_slice(&res[len..]);
					res = tmp;
					found = true;
					break;
				}
			}
			!found
		});
	}
	res
}

#[cfg(test)]
mod test {
	use super::shortest_superstring;
	#[test]
	fn test_assemble() {
		let input = vec![
			b"ATTAGACCTG".to_vec(),
			b"CCTGCCGGAA".to_vec(),
			b"AGACCTGCCG".to_vec(),
			b"GCCGGAATAC".to_vec(),
			b"ACCTGCCGGA".to_vec(),
			b"GGAATACACG".to_vec(),
		];
		let output = b"ATTAGACCTGCCGGAATACACG".to_vec();

		let res = shortest_superstring(input);
		assert_eq!(std::str::from_utf8(&output), std::str::from_utf8(&res));
	}
}
