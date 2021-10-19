pub(super) fn amino_codon_combinations(amino: u8) -> u8 {
	match amino {
		b'A' => 4,
		b'C' => 2,
		b'D' => 2,
		b'E' => 2,
		b'F' => 2,
		b'G' => 4,
		b'H' => 2,
		b'I' => 3,
		b'K' => 2,
		b'L' => 6,
		b'M' => 1,
		b'N' => 2,
		b'P' => 4,
		b'Q' => 2,
		b'R' => 6,
		b'S' => 6,
		b'T' => 4,
		b'V' => 4,
		b'W' => 1,
		b'Y' => 2,
		b'*' => 3,
		_ => 0,
	}
}

pub fn potential_mrna_strings_from_protein(prot: &[u8], modulo: u64) -> u64 {
	prot.iter()
		.fold(u64::from(amino_codon_combinations(b'*')), |acc, x| {
			(acc * u64::from(amino_codon_combinations(*x))) % modulo
		})
}

pub fn permutation(k: u8) -> Vec<Vec<u8>> {
	let mut values: Vec<u8> = (1..=k).collect();
	let k = k as usize;
	let mut c = vec![0; k];
	let mut out = vec![values.clone()];

	let mut i: usize = 1;

	while i < k {
		if c[i] < i {
			if i % 2 == 0 {
				values.swap(0, i);
			} else {
				values.swap(c[i], i);
			}
			out.push(values.clone());

			c[i] += 1;
			i = 1;
		} else {
			c[i] = 0;
			i += 1;
		}
	}

	out
}

pub fn partial_permutation(n: u64, k: u64, m: u64) -> u64 {
	assert!(n >= k);
	if n == k {
		modulo_factorial(n, m)
	} else {
		let mut accum = 1;
		for i in 0..k {
			accum *= (n - i) % m;
		}
		accum % m
	}
}

pub fn modulo_factorial(num: u64, m: u64) -> u64 {
	(1..=num).fold(1, |acc, v| (acc * v) % m)
}

pub fn factorial(num: u64) -> u64 {
	(1..=num).product()
}

#[cfg(test)]
mod test {
	use super::{modulo_factorial, partial_permutation};

	#[test]
	fn test_mrna_protein() {
		let seq = vec![b'M', b'A'];
		assert_eq!(2, super::potential_mrna_strings_from_protein(&seq, 10));
	}

	#[test]
	fn test_permutation() {
		let mut output = vec![
			vec![1, 2, 3],
			vec![1, 3, 2],
			vec![2, 1, 3],
			vec![2, 3, 1],
			vec![3, 1, 2],
			vec![3, 2, 1],
		];
		output.sort();
		let mut res = super::permutation(3);
		res.sort();

		assert_eq!(res, output);
	}

	#[test]
	fn test_partial_perm() {
		assert_eq!(partial_permutation(6, 6, 700), 20);
		assert_eq!(partial_permutation(6, 1, 1000), 6);
		assert_eq!(
			partial_permutation(2, 2, 1000000),
			modulo_factorial(2, 10000)
		);
		assert_eq!(partial_permutation(21, 7, 1_000_000), 51200);
	}

	#[test]
	fn test_factorial() {
		assert_eq!(modulo_factorial(6, 1000), 720);
		assert_eq!(modulo_factorial(6, 700), 20);
	}
}
