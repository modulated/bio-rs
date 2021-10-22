pub fn longest_common_sequence<T: PartialEq + Copy>(left: &[T], right: &[T]) -> Vec<T> {
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

pub fn shortest_common_supersequence<T>(left: &[T], right: &[T]) -> Vec<T>
where
	T: PartialEq	
{
	vec![]
}

#[cfg(test)]
mod test {
	use crate::alignment::lcs::shortest_common_supersequence;

use super::longest_common_sequence;

	#[test]
	fn lcs() {
		let a_1 = "ACGTACG".as_bytes();
		let b_1 = "ATACAGTACGTA".as_bytes();

		let out_1 = longest_common_sequence(&a_1, &b_1);
		assert_eq!(out_1, "ACGTACG".as_bytes());

		let a_2 = "AACCTAGG".as_bytes();
		let b_2 = "ACACTGTGA".as_bytes();

		let out_2 = longest_common_sequence(&a_2, &b_2);
		assert_eq!(out_2, "AACTGG".as_bytes());
	}

	#[test]
	fn scs() {
		let a_1 = "ACGTACG".as_bytes();
		let b_1 = "CAG".as_bytes();
		let res_1 = "ACGTACG".as_bytes();

		let out_1 = shortest_common_supersequence(&a_1, &b_1);
		assert_eq!(out_1, res_1);

		let a_2 = "AGGTAC".as_bytes();
		let b_2 = "GCTCATC".as_bytes();
		let res_2 = "AGCGTCATC".as_bytes();

		let out_2 = shortest_common_supersequence(&a_2, &b_2);
		assert_eq!(out_2, res_2);
	}
}
