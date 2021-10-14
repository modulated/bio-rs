use crate::Seq;

#[allow(dead_code)]
#[derive(PartialEq, Debug)]
pub struct FASTA {
	pub name: String,
	pub seq: Seq,
}

pub fn parse_string_to_vec_of_fasta(input: &str) -> Vec<FASTA> {
	let mut res = vec![];
	let arr: Vec<&str> = input.split('>').skip(1).collect();

	for s in arr {
		let v: Vec<String> = s
			.splitn(2, '\n')
			.map(|n| n.split_whitespace().collect::<Vec<&str>>().join(""))
			.collect();

		let f = FASTA {
			name: v[0].clone(),
			seq: Seq::new(&v[1]),
		};
		res.push(f);
	}

	res
}

#[cfg(test)]
mod test {
	use crate::{fasta::parse_string_to_vec_of_fasta, fasta::FASTA, Seq};
	#[test]
	fn parse() {
		let input = r#">Rosalind_6404
        CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAGGCTTCCGGCCTTCCC
        TCCCACTAATAATTCTGAGG
        >Rosalind_5959
        CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCGCTCCGCCGAAGGTCT
        ATATCCATTTGTCAGCAGACACGC
        >Rosalind_0808
        CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTCAGACCAGCCCGGAC
        TGGGAACCTGCGGGCAGTAGGTGGAAT"#;

		let output = vec![
			FASTA {
				name: "Rosalind_6404".to_string(),
				seq: Seq::new(
					"CCTGCGGAAGATCGGCACTAGAATAGCCAGAACCGTTTCTCTGAG
                GCTTCCGGCCTTCCCTCCCACTAATAATTCTGAGG",
				),
			},
			FASTA {
				name: "Rosalind_5959".to_string(),
				seq: Seq::new(
					"CCATCGGTAGCGCATCCTTAGTCCAATTAAGTCCCTATCCAGGCG
                CTCCGCCGAAGGTCTATATCCATTTGTCAGCAGACACGC",
				),
			},
			FASTA {
				name: "Rosalind_0808".to_string(),
				seq: Seq::new(
					"CCACCCTCGTGGTATGGCTAGGCATTCAGGAACCGGAGAACGCTTC
                AGACCAGCCCGGACTGGGAACCTGCGGGCAGTAGGTGGAAT",
				),
			},
		];

		assert_eq!(output, parse_string_to_vec_of_fasta(input));
	}
}
