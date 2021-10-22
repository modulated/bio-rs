use crate::Seq;

#[derive(PartialEq, Debug)]
pub struct ProteinMotif(Vec<AminoMotif>);

#[derive(PartialEq, Debug)]
pub enum AminoMotif {
	Amino(u8),
	Either(u8, u8),
	Except(u8),
}

impl ProteinMotif {
	#[must_use]
	pub fn new(string: &str) -> Self {
		let mut vec = vec![];
		let mut iter = string.bytes();
		while let Some(c) = iter.next() {
			match c {
				b'[' => {
					let a = iter.next().unwrap();
					let b = iter.next().unwrap();
					assert_eq!(iter.next().unwrap(), b']');
					vec.push(AminoMotif::Either(a, b));
				}
				b'{' => {
					let a = iter.next().unwrap();
					assert_eq!(iter.next().unwrap(), b'}');
					vec.push(AminoMotif::Except(a));
				}
				_ => {
					vec.push(AminoMotif::Amino(c));
				}
			}
		}

		Self(vec)
	}

	#[must_use]
	pub fn find_in(&self, seq: &Seq) -> Vec<usize> {
		let mut res = vec![];
		let mut i = 0;

		while i < (seq.len() - self.0.len() + 1) {
			let mut j = 0;

			while j < self.0.len() {
				let seq_amino = seq.0[i + j];
				match self.0[j] {
					AminoMotif::Amino(a) => {
						if a == seq_amino {
							j += 1;
						} else {
							break;
						}
					}
					AminoMotif::Either(a, b) => {
						if a == seq_amino || b == seq_amino {
							j += 1;
						} else {
							break;
						}
					}
					AminoMotif::Except(a) => {
						if a == seq_amino {
							break;
						}
						j += 1;
					}
				};
				if j == self.0.len() {
					res.push(i + 1);
				}
			}
			i += 1;
		}

		res
	}
}

#[cfg(test)]
mod test {
	use super::{AminoMotif, ProteinMotif};

	#[test]
	fn protmotif_from_string() {
		let input = "N{P}[ST]{P}";
		let output = ProteinMotif(vec![
			AminoMotif::Amino(b'N'),
			AminoMotif::Except(b'P'),
			AminoMotif::Either(b'S', b'T'),
			AminoMotif::Except(b'P'),
		]);

		assert_eq!(output, ProteinMotif::new(&input));
	}

	#[test]
	fn find_motif_in_prot() {
		use crate::Seq;

		let input_1 = "MKNKFKTQEELVNHLKTVGFVFANSEIYNGLANAWDYGPLGVLLKNNLKNLWWKEFVTKQ
        KDVVGLDSAIILNPLVWKASGHLDNFSDPLIDCKNCKARYRADKLIESFDENIHIAENSS
        NEEFAKVLNDYEISCPTCKQFNWTEIRHFNLMFKTYQGVIEDAKNVVYLRPETAQGIFVN
        FKNVQRSMRLHLPFGIAQIGKSFRNEITPGNFIFRTREFEQMEIEFFLKEESAYDIFDKY
        LNQIENWLVSACGLSLNNLRKHEHPKEELSHYSKKTIDFEYNFLHGFSELYGIAYRTNYD
        LSVHMNLSKKDLTYFDEQTKEKYVPHVIEPSVGVERLLYAILTEATFIEKLENDDERILM
        DLKYDLAPYKIAVMPLVNKLKDKAEEIYGKILDLNISATFDNSGSIGKRYRRQDAIGTIY
        CLTIDFDSLDDQQDPSFTIRERNSMAQKRIKLSELPLYLNQKAHEDFQRQCQK";
		let output_1 = vec![85, 118, 142, 306, 395];

		let pm = super::ProteinMotif::new("N{P}[ST]{P}");
		let seq_1 = Seq::new(input_1);
		let res_1 = pm.find_in(&seq_1);
		assert_eq!(res_1, output_1);
	}

	#[test]
	#[ignore]
	fn find_motif_in_prot_from_database() {
		
		let pm = super::ProteinMotif::new("N{P}[ST]{P}");

		let fasta_2 = crate::formats::FASTA::from_uniprot_id("P07204_TRBM_HUMAN");
		let output_2 = vec![47, 115, 116, 382, 409];
		let res_2 = pm.find_in(&fasta_2.seq);
		assert_eq!(output_2, res_2);

		let fasta_3 = crate::formats::FASTA::from_uniprot_id("P20840_SAG1_YEAST");
		let output_3 = vec![79, 109, 135, 248, 306, 348, 364, 402, 485, 501, 614];
		let res_3 = pm.find_in(&fasta_3.seq);
		assert_eq!(output_3, res_3);
	}
}
