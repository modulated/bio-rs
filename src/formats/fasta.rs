use crate::Seq;
use ureq;

#[derive(PartialEq, Debug)]
pub struct FASTA {
	pub name: String,
	pub seq: Seq,
}

impl FASTA {
	pub fn from_uniprot_id(id: &str) -> Self {
		let path = format!("https://www.uniprot.org/uniprot/{}.fasta", id);
		let r = ureq::get(&path).call().unwrap().into_string().unwrap();
		let split = &r[1..].split_once('\n');
		let (name, seq) = split.unwrap();
		Self {
			name: name.to_string(),
			seq: Seq::new(seq),
		}
	}

	pub fn from_ena_id(id: &str) -> Self {
		let path = format!("https://www.ebi.ac.uk/Tools/dbfetch/dbfetch?db=ena_sequence&id={}&format=fasta&style=raw", id);
		let r = ureq::get(&path).call().unwrap().into_string().unwrap();
		let split = &r[1..].split_once('\n');
		let (name, seq) = split.unwrap();
		Self {
			name: name.to_string(),
			seq: Seq::new(seq),
		}
	}
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
	use super::{parse_string_to_vec_of_fasta, FASTA};
	use crate::Seq;
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

	#[test]
	#[ignore]
	fn uniprot() {
		let seq = "MKNKFKTQEELVNHLKTVGFVFANSEIYNGLANAWDYGPLGVLLKNNLKNLWWKEFVTKQ
		KDVVGLDSAIILNPLVWKASGHLDNFSDPLIDCKNCKARYRADKLIESFDENIHIAENSS
		NEEFAKVLNDYEISCPTCKQFNWTEIRHFNLMFKTYQGVIEDAKNVVYLRPETAQGIFVN
		FKNVQRSMRLHLPFGIAQIGKSFRNEITPGNFIFRTREFEQMEIEFFLKEESAYDIFDKY
		LNQIENWLVSACGLSLNNLRKHEHPKEELSHYSKKTIDFEYNFLHGFSELYGIAYRTNYD
		LSVHMNLSKKDLTYFDEQTKEKYVPHVIEPSVGVERLLYAILTEATFIEKLENDDERILM
		DLKYDLAPYKIAVMPLVNKLKDKAEEIYGKILDLNISATFDNSGSIGKRYRRQDAIGTIY
		CLTIDFDSLDDQQDPSFTIRERNSMAQKRIKLSELPLYLNQKAHEDFQRQCQK"
			.chars()
			.filter(|x| x.is_alphabetic())
			.collect::<String>();

		let r = FASTA::from_uniprot_id("B5ZC00");
		assert_eq!(r.name, "sp|B5ZC00|SYG_UREU1 Glycine--tRNA ligase OS=Ureaplasma urealyticum serovar 10 (strain ATCC 33699 / Western) OX=565575 GN=glyQS PE=3 SV=1");
		assert_eq!(r.seq.to_string(), seq);
	}

	#[test]
	#[ignore]
	fn ena() {
		let r = FASTA::from_ena_id("LT599825.1");
		assert_eq!(r.name, "ENA|LT599825|LT599825.1 Escherichia coli isolate E. coli NRZ14408 genome assembly, chromosome: NRZ14408_C");
		assert_eq!(r.seq.len(), 5344876);
	}
}
