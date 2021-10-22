// TODO: Could be optimised - currently O(n * m) time
pub fn substring<N, H>(needle: &[N], haystack: &[H]) -> Vec<usize>
where
	N: PartialEq,
	H: PartialEq<N>,
{
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

pub fn subsequence<N, H>(needle: &[N], haystack: &[H]) -> Vec<usize>
where
	N: PartialEq,
	H: PartialEq<N>,
{
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
	use crate::alignment::substring::{subsequence, substring};

	#[test]
	fn test_substring() {
		let hs = vec![1, 2, 3, 4, 5, 6, 4, 5, 6];
		let nd = vec![4, 5, 6];

		assert_eq!(vec![4, 7], substring(&nd, &hs));
	}

	#[test]
	fn test_subsequence() {
		let hs = b"ACGCGCGTGACG";
		let nd = b"GTA";
		let res = vec![3, 8, 10];

		assert_eq!(res, subsequence(nd, hs));
	}
}
