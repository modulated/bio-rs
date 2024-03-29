use crate::formats::FASTAVec;
/// Returns a vec containing edges (names of fasta seq) of a directed graph of overlapping sequences
#[must_use]
pub fn create_graph(seqs: &FASTAVec, overlap_size: usize) -> Vec<(String, String)> {
	let mut output = vec![];

	for (i, s1) in {
		let this = &seqs;
		this.0.iter()
	}
	.enumerate()
	{
		for s2 in &seqs[i + 1..] {
			if s1.seq.suffix_overlap(&s2.seq, overlap_size) {
				output.push((s1.name.to_string(), s2.name.to_string()));
			}
			if s2.seq.suffix_overlap(&s1.seq, overlap_size) {
				output.push((s2.name.to_string(), s1.name.to_string()));
			}
		}
	}

	output
}

#[cfg(test)]
mod test {
	#[test]
	fn directed_overlap_graph() {
		let input = ">Rosalind_0498
        AAATAAA
        >Rosalind_2391
        AAATTTT
        >Rosalind_2323
        TTTTCCC
        >Rosalind_0442
        AAATCCC
        >Rosalind_5013
        GGGTGGG";
		let output: Vec<(String, String)> = vec![
			("Rosalind_0498", "Rosalind_2391"),
			("Rosalind_0498", "Rosalind_0442"),
			("Rosalind_2391", "Rosalind_2323"),
		]
		.into_iter()
		.map(|(a, b)| (a.to_string(), b.to_string()))
		.collect();

		let fv = crate::FASTAVec::from_string(input);

		let g = super::create_graph(&fv, 3);
		assert_eq!(output, g);
	}
}
