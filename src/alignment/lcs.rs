use crate::Seq;

pub fn longest_common_sequence(left: &Seq, right: &Seq) -> Seq {
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
		return Seq::default();
	}

	let mut i = matrix.len() - 1;
	let mut j = matrix[0].len() - 1;
	let mut output: Vec<u8> = Vec::with_capacity(lcs_len);

	while i > 0 && j > 0 {
		if left.0[i - 1] == right.0[j - 1] {
			output.push(left.0[i - 1]);
			i -= 1;
			j -= 1;
		} else if matrix[i - 1][j] >= matrix[i][j - 1] {
			i -= 1;
		} else {
			j -= 1;
		}
	}

	output.reverse();
	Seq::from_bytes(&output)
}

#[cfg(test)]
mod test {
	use super::longest_common_sequence;
	use crate::Seq;

	#[test]
	fn lcs() {
		let a_1 = Seq::new("ACGTACG");
		let b_1 = Seq::new("ATACAGTACGTA");

		let out_1 = longest_common_sequence(&a_1, &b_1);
		assert_eq!(out_1.to_string(), "ACGTACG");

		let a_2 = Seq::new("AACCTAGG");
		let b_2 = Seq::new("ACACTGTGA");

		let out_2 = longest_common_sequence(&a_2, &b_2);
		assert_eq!(out_2.to_string(), "AACTGG");
	}
}
