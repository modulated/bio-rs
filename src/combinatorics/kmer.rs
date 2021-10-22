use core::hash::Hash;
use std::collections::HashMap;

pub fn ordered_kmer_permutations<T: Ord + Sized + Copy + Eq + Hash + std::fmt::Debug>(
	alphabet: &[T],
	k: usize,
) -> Vec<Vec<T>> {
	if k == 1 {
		return alphabet.iter().copied().map(|x| vec![x]).collect();
	}

	let mut vec = Vec::new();
	let recurse = ordered_kmer_permutations(alphabet, k - 1);
	for i in recurse {
		for j in alphabet {
			let mut combin = i.clone();
			combin.push(*j);
			vec.push(combin);
		}
	}
	vec
}

pub fn composition_map<T: Ord + Sized + Copy + Eq + Hash + std::fmt::Debug>(
	string: &[T],
	k: usize,
) -> HashMap<Vec<T>, usize> {
	let mut out = HashMap::new();

	for s in string.windows(k) {
		assert_eq!(s.len(), k);
		let e = out.entry(s.to_vec()).or_insert(0);
		*e += 1;
	}

	out
}

pub fn lexographical_kmer_composition<T: Ord + Sized + Copy + Eq + Hash + std::fmt::Debug>(
	string: &[T],
	alphabet: &[T],
	k: usize,
) -> Vec<usize> {
	let mut out = vec![];
	let map = composition_map(string, k);
	// println!("{:#?}", map);
	for l in ordered_kmer_permutations(alphabet, k) {
		assert_eq!(l.len(), k);
		out.push(map.get(&l).unwrap_or(&0).to_owned());
	}
	out
}

#[cfg(test)]
mod test {

	#[test]
	fn ordered_kmer_permutations() {
		let alph = vec!['A', 'C', 'G', 'T'];
		let perms = vec![
			"AA", "AC", "AG", "AT", "CA", "CC", "CG", "CT", "GA", "GC", "GG", "GT", "TA", "TC",
			"TG", "TT",
		];
		let res: Vec<String> = super::ordered_kmer_permutations::<char>(&alph, 2)
			.iter()
			.map(|x| x.iter().collect())
			.collect();

		assert_eq!(res, perms);
	}

	#[test]
	fn kmer_composition() {
		let input = "CTTCGAAAGTTTGGGCCGAGTCTTACAGTCGGTCTTGAAGCAAAGTAACGAACTCCACGG
		CCCTGACTACCGAACCAGTTGTGAGTACTCAACTGGGTGAGAGTGCAGTCCCTATTGAGT
		TTCCGAGACTCACCGGGATTTTCGATCCAGCCTCAGTCCAGTCTTGTGGCCAACTCACCA
		AATGACGTTGGAATATCCCTGTCTAGCTCACGCAGTACTTAGTAAGAGGTCGCTGCAGCG
		GGGCAAGGAGATCGGAAAATGTGCTCTATATGCGACTAAAGCTCCTAACTTACACGTAGA
		CTTGCCCGTGTTAAAAACTCGGCTCACATGCTGTCTGCGGCTGGCTGTATACAGTATCTA
		CCTAATACCCTTCAGTTCGCCGCACAAAAGCTGGGAGTTACCGCGGAAATCACAG";

		let output: Vec<usize> = vec![
			4, 1, 4, 3, 0, 1, 1, 5, 1, 3, 1, 2, 2, 1, 2, 0, 1, 1, 3, 1, 2, 1, 3, 1, 1, 1, 1, 2, 2,
			5, 1, 3, 0, 2, 2, 1, 1, 1, 1, 3, 1, 0, 0, 1, 5, 5, 1, 5, 0, 2, 0, 2, 1, 2, 1, 1, 1, 2,
			0, 1, 0, 0, 1, 1, 3, 2, 1, 0, 3, 2, 3, 0, 0, 2, 0, 8, 0, 0, 1, 0, 2, 1, 3, 0, 0, 0, 1,
			4, 3, 2, 1, 1, 3, 1, 2, 1, 3, 1, 2, 1, 2, 1, 1, 1, 2, 3, 2, 1, 1, 0, 1, 1, 3, 2, 1, 2,
			6, 2, 1, 1, 1, 2, 3, 3, 3, 2, 3, 0, 3, 2, 1, 1, 0, 0, 1, 4, 3, 0, 1, 5, 0, 2, 0, 1, 2,
			1, 3, 0, 1, 2, 2, 1, 1, 0, 3, 0, 0, 4, 5, 0, 3, 0, 2, 1, 1, 3, 0, 3, 2, 2, 1, 1, 0, 2,
			1, 0, 2, 2, 1, 2, 0, 2, 2, 5, 2, 2, 1, 1, 2, 1, 2, 2, 2, 2, 1, 1, 3, 4, 0, 2, 1, 1, 0,
			1, 2, 2, 1, 1, 1, 5, 2, 0, 3, 2, 1, 1, 2, 2, 3, 0, 3, 0, 1, 3, 1, 2, 3, 0, 2, 1, 2, 2,
			1, 2, 3, 0, 1, 2, 3, 1, 1, 3, 1, 0, 1, 1, 3, 0, 2, 1, 2, 2, 0, 2, 1, 1,
		];

		let alpha = vec!['A', 'C', 'G', 'T'];
		let mut sanitized_input = input.chars().collect::<Vec<_>>();
		sanitized_input.retain(|x| x.is_alphanumeric());

		let res = super::lexographical_kmer_composition(&sanitized_input, &alpha, 4);

		assert_eq!(output, res);
	}
}
