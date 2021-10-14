pub fn ordered_kmer_permutations<T>(alphabet: &[T], n: usize) -> Vec<Vec<T>>
where
	T: PartialOrd + std::fmt::Debug + Sized + Copy,
{
	if n == 1 {
		return alphabet.iter().cloned().map(|x| vec![x]).collect();
	}

	let mut vec = Vec::new();
	let recurse = ordered_kmer_permutations(alphabet, n - 1);
	for i in recurse {
		for j in alphabet {
			let mut combin = i.clone();
			combin.push(*j);
			vec.push(combin);
		}
	}
	vec
}

#[cfg(test)]
mod test {
	use super::ordered_kmer_permutations;

	#[test]
	fn test_ordered_kmer_permutations() {
		let alph = vec!['A', 'C', 'G', 'T'];
		let perms = vec![
			"AA", "AC", "AG", "AT", "CA", "CC", "CG", "CT", "GA", "GC", "GG", "GT", "TA", "TC",
			"TG", "TT",
		];
		let res: Vec<String> = ordered_kmer_permutations::<char>(&alph, 2)
			.iter()
			.map(|x| x.into_iter().collect())
			.collect();

		assert_eq!(res, perms);
	}
}
