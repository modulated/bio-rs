use crate::Seq;

#[must_use]
pub fn reverse(seq: &Seq, min: usize, max: usize) -> Vec<(usize, usize)> {
	let mut out: Vec<(usize, usize)> = vec![];

	for i in min / 2..=seq.len() - min / 2 {
		for j in 1..=max / 2 {
			if seq.0[i - j] != nuc_complement(seq.0[i - 1 + j]) {
				break;
			}
			if j >= min / 2 {
				out.push((i - j + 1, j * 2));
			}
			if j + i + 1 > seq.len() || j + 1 > i {
				break;
			}
		}
	}

	out
}

fn nuc_complement(n: u8) -> u8 {
	#[allow(unused_assignments)]
	let mut out = 0;
	match n {
		b'A' | b'a' => out = b'T',
		b'C' | b'c' => out = b'G',
		b'G' | b'g' => out = b'C',
		b'T' | b't' | b'U' | b'u' => out = b'A',
		_ => panic!("Unexpected character {}", n),
	};

	out
}

#[cfg(test)]
mod test {
	use super::reverse;
	use crate::Seq;

	#[test]
	fn test_rev_palindrome() {
		let input = "TCAATGCATGCGGGTCTATATGCAT";
		let output = vec![
			(4, 6),
			(5, 4),
			(6, 6),
			(7, 4),
			(17, 4),
			(18, 4),
			(20, 6),
			(21, 4),
		];

		let mut res = reverse(&Seq::new(input), 4, 12);
		res.sort_unstable();
		assert_eq!(res, output);
	}

	#[test]
	fn test_rev_palindrome_2() {
		let input = "TATATA";
		let output = vec![(1, 4), (1, 6), (2, 4), (3, 4)];

		let mut res = reverse(&Seq::new(input), 4, 12);
		res.sort_unstable();
		assert_eq!(res, output);
	}

	#[test]
	fn test_rev_palindrome_3() {
		let input = "TTTAAATTTAAA";
		let output = vec![
			(1, 6),
			(1, 12),
			(2, 4),
			(2, 10),
			(3, 8),
			(4, 6),
			(5, 4),
			(7, 6),
			(8, 4),
		];

		let mut res = reverse(&Seq::new(input), 4, 12);
		res.sort_unstable();
		assert_eq!(res, output);
	}
}
