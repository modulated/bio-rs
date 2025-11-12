use crate::{GenBank, Seq, FASTQ};
use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub struct FASTA {
	pub name: String,
	pub seq: Seq,
}

impl FASTA {
	#[must_use]
	pub fn from_uniprot_id(id: &str) -> Self {
		let path = format!("https://www.uniprot.org/uniprot/{id}.fasta");
		let r = ureq::get(&path).call().unwrap().body_mut().read_to_string().unwrap();
		let split = &r[1..].split_once('\n');
		let (name, seq) = split.unwrap();
		Self {
			name: name.to_string(),
			seq: Seq::new(seq),
		}
	}

	#[must_use]
	pub fn from_ena_id(id: &str) -> Self {
		let path = format!("https://www.ebi.ac.uk/Tools/dbfetch/dbfetch?db=ena_sequence&id={id}&format=fasta&style=raw");
		let r = ureq::get(&path).call().unwrap().body_mut().read_to_string().unwrap();
		let split = &r[1..].split_once('\n');
		let (name, seq) = split.unwrap();
		Self {
			name: name.to_string(),
			seq: Seq::new(seq),
		}
	}

	#[must_use]
	pub fn from_file(file: &str) -> Self {
		let cont = std::fs::read_to_string(file).unwrap();
		let (name, seq) = cont
			.trim()
			.trim_start_matches('>')
			.split_once('\n')
			.unwrap();

		Self {
			name: name.trim().to_string(),
			seq: Seq::new(seq),
		}
	}
}

impl Display for FASTA {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let seq = self
			.seq
			.0
			.chunks(70)
			.map(|x| String::from_utf8(x.to_vec()).unwrap())
			.collect::<Vec<_>>()
			.join("\n");
		write!(f, "{}\n{}", self.name, seq)
	}
}

impl From<FASTQ> for FASTA {
	fn from(fastq: FASTQ) -> Self {
		Self {
			name: fastq.name,
			seq: fastq.seq,
		}
	}
}

impl From<GenBank> for FASTA {
	fn from(genbank: GenBank) -> Self {
		Self {
			name: format!(
				"{} {}",
				genbank.0.name.unwrap_or_default(),
				genbank.0.definition.unwrap_or_default()
			),
			seq: Seq::from_bytes(&genbank.0.seq),
		}
	}
}

#[cfg(test)]
mod test {
	use crate::{Seq, FASTA, FASTQ};

	#[test]
	fn from_file() {
		let e = FASTA::from_file("benches/salmonella.fasta");
		assert_eq!(e.name, "JYPS01000003.1 Salmonella enterica strain CVM 43749 43749_contig_3, whole genome shotgun sequence");
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
		assert_eq!(r.seq.len(), 5_344_876);
	}

	#[test]
	fn from_fastq() {
		let fq = FASTQ::new(
			r"@SEQ_ID
GATTTGGGGTTCAAAGCAGTATCGATCAAATAGTAAATCCATTTGTTCAACTCACAGTTT
+
!*((((***+))%%%++)(%%%%).1***-+*****))**55CCF>>>>>>CCCCCCC65",
		);

		let fs: FASTA = fq.into();

		assert_eq!(fs.name, "SEQ_ID");
		assert_eq!(
			fs.seq,
			Seq::new("GATTTGGGGTTCAAAGCAGTATCGATCAAATAGTAAATCCATTTGTTCAACTCACAGTTT")
		);
	}
}
