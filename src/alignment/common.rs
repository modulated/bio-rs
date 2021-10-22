use std::fmt::Debug;

pub fn longest_common_subsequence<T: PartialEq + Copy + Debug>(left: &[T], right: &[T]) -> Vec<T> {
	let left_len = left.len();
	let right_len = right.len();

	let mut matrix = vec![vec![0; right_len + 1]; left_len + 1];

	for i in 1..=left_len {
		for j in 1..=right_len {
			if left[i - 1] == right[j - 1] {
				matrix[i][j] = matrix[i - 1][j - 1] + 1;
			} else {
				matrix[i][j] = std::cmp::max(matrix[i][j - 1], matrix[i - 1][j]);
			}
		}
	}

	let lcs_len = matrix[left_len][right_len];

	if lcs_len == 0 {
		return Vec::default();
	}

	let mut i = matrix.len() - 1;
	let mut j = matrix[0].len() - 1;
	let mut output: Vec<T> = Vec::with_capacity(lcs_len);

	while i > 0 && j > 0 {
		if left[i - 1] == right[j - 1] {
			output.push(left[i - 1]);
			i -= 1;
			j -= 1;
		} else if matrix[i - 1][j] >= matrix[i][j - 1] {
			i -= 1;
		} else {
			j -= 1;
		}
	}

	output.reverse();
	output
}

pub fn shortest_common_supersequence<T: PartialEq + Copy + Debug>(
	left: &[T],
	right: &[T],
) -> Vec<T> {
	let mut out = vec![];

	let left_len = left.len();
	let right_len = right.len();

	let lcsq_string = longest_common_subsequence(left, right);

	let mut i = 0;
	let mut j = 0;

	for char in lcsq_string {
		if i < left_len {
			while left[i] != char {
				out.push(left[i]);
				i += 1;
			}
			i += 1;
		}
		if j < right_len {
			while right[j] != char {
				out.push(right[j]);
				j += 1;
			}
			j += 1;
		}
		out.push(char);
	}

	if i < left_len {
		out.extend_from_slice(&left[i..]);
	}
	if j < right_len {
		out.extend_from_slice(&right[j..]);
	}

	out
}

#[cfg(test)]
mod test {
	use super::{longest_common_subsequence, shortest_common_supersequence};

	#[test]
	fn lcs() {
		let a_1 = "ACGTACG".as_bytes();
		let b_1 = "ATACAGTACGTA".as_bytes();

		let out_1 = longest_common_subsequence(&a_1, &b_1);
		assert_eq!(out_1, "ACGTACG".as_bytes());

		let a_2 = "AACCTAGG".as_bytes();
		let b_2 = "ACACTGTGA".as_bytes();

		let out_2 = longest_common_subsequence(&a_2, &b_2);
		assert_eq!(out_2, "AACTGG".as_bytes());
	}

	#[test]
	fn scs1() {
		let a = "ACGTACG".chars().collect::<Vec<_>>();
		let b = "CAG".chars().collect::<Vec<_>>();
		let res = "ACGTACG".chars().collect::<Vec<_>>();

		let out = shortest_common_supersequence(&a, &b);
		assert_eq!(out, res);
	}
	#[test]
	fn scs2() {
		let a = "ATCTGAT".chars().collect::<Vec<_>>();
		let b = "TGCATA".chars().collect::<Vec<_>>();
		let res = "ATGCATGAT".chars().collect::<Vec<_>>();

		let out = shortest_common_supersequence(&a, &b);
		assert_eq!(out, res);
	}

	#[test]
	fn scs3() {
		let a = "ABCDEFGH".chars().collect::<Vec<_>>();
		let b = "AXYZB".chars().collect::<Vec<_>>();
		let res = "AXYZBCDEFGH".chars().collect::<Vec<_>>();

		let out = shortest_common_supersequence(&a, &b);
		assert_eq!(out, res);
	}
}
