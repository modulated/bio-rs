// TODO: Could be optimised - currently O(n * m) time
pub fn substring<N: PartialEq,H: PartialEq<N>>(needle: &[N], haystack: &[H]) -> Vec<usize> {
	let mut out = vec![];

	if needle.len() > haystack.len() {
		return out;
	}

	for i in 0..=haystack.len() - needle.len() {
		for j in 0..needle.len() {
			if haystack[i + j] != needle[j] {
				break;
			}

			if j == needle.len() - 1 {
				out.push(i + 1);
			}
		}
	}

	out
}

pub fn contains<N: PartialEq,H: PartialEq<N>>(needle: &[N], haystack: &[H]) -> bool {
	if needle.len() > haystack.len() {
		return false;
	}

	for i in 0..=haystack.len() - needle.len() {
		for j in 0..needle.len() {
			if haystack[i + j] != needle[j] {
				break;
			}

			if j == needle.len() - 1 {
				return true;
			}
		}
	}
	false
}

pub fn subsequence<N: PartialEq,H: PartialEq<N>>(needle: &[N], haystack: &[H]) -> Vec<usize> {
	let mut out = vec![];

	if needle.len() > haystack.len() {
		return out;
	}

	let mut n_idx = 0;

	for (i, c) in haystack.iter().enumerate() {
		if *c == needle[n_idx] {
			n_idx += 1;
			out.push(i + 1);
			if n_idx == needle.len() {
				break;
			}
		}
	}

	out
}

#[cfg(test)]
mod test {
	use crate::alignment::substring::*;

	#[test]
	fn test_substring() {
		let hs = vec![1, 2, 3, 4, 5, 6, 4, 5, 6];
		let nd = vec![4, 5, 6];

		assert_eq!(vec![4, 7], substring(&nd, &hs));
	}

	#[test]
	fn test_contains() {
		let hs = vec![1, 2, 3, 4, 5, 6, 4, 5, 6];
		let nd = vec![4, 5, 6];
		let nd2 = vec![10, 11];

		assert!(contains(&nd, &hs));
		assert!(!contains(&nd2, &hs));
		assert!(!contains(&hs, &nd));
	}

	#[test]
	fn test_subsequence() {
		let hs = b"ACGCGCGTGACG";
		let nd = b"GTA";
		let res = vec![3, 8, 10];

		assert_eq!(res, subsequence(nd, hs));
	}
}
