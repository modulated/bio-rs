use num::BigUint;

pub(super) const fn amino_codon_combinations(amino: u8) -> u8 {
	match amino {
		b'M' | b'W' => 1,
		b'C' | b'D' | b'E' | b'F' | b'K' | b'N' | b'Y' | b'Q' | b'H' => 2,
		b'I' | b'*' => 3,
		b'G' | b'P' | b'T' | b'A' | b'V' => 4,
		b'L' | b'R' | b'S' => 6,
		_ => 0,
	}
}

#[must_use]
pub fn potential_mrna_strings_from_protein(prot: &[u8], modulo: u64) -> u64 {
	prot.iter()
		.fold(u64::from(amino_codon_combinations(b'*')), |acc, x| {
			(acc * u64::from(amino_codon_combinations(*x))) % modulo
		})
}

#[must_use]
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

#[must_use]
pub fn ncr(n: u64, r: u64) -> BigUint {
    let r = r.min(n - r);
    if r == 0 {
        return BigUint::from(1_u64);
    }
    let numerator: BigUint = ((n - r + 1)..=n).product();
    let denominator: BigUint = (1..=r).product();
    numerator / denominator
}

#[must_use]
pub fn partial_permutation_modulo(n: u64, k: u64, m: u64) -> u64 {
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

#[must_use]
pub fn modulo_factorial(num: u64, m: u64) -> u64 {
	(1..=num).fold(1, |acc, v| (acc * v) % m)
}

#[must_use]
pub fn factorial(num: u64) -> u64 {
	(1..=num).product()
}

#[cfg(test)]
mod test {
	use super::{modulo_factorial, partial_permutation_modulo};

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
		assert_eq!(partial_permutation_modulo(6, 6, 700), 20);
		assert_eq!(partial_permutation_modulo(6, 1, 1000), 6);
		assert_eq!(
			partial_permutation_modulo(2, 2, 1_000_000),
			modulo_factorial(2, 10000)
		);
		assert_eq!(partial_permutation_modulo(21, 7, 1_000_000), 51200);
	}

	#[test]
	fn test_factorial() {
		assert_eq!(modulo_factorial(6, 1000), 720);
		assert_eq!(modulo_factorial(6, 700), 20);
	}
}
