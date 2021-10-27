use crate::{BioResult, Seq};

#[derive(Debug)]
pub struct TranslationBuilder<'a> {
	pub(super) seq: &'a Seq,
	pub(super) transl_table: u8,
	pub(super) to_stop: bool,
	pub(super) stop_symbol: u8,
}

impl<'a> TranslationBuilder<'a> {
	pub fn transl_table(&'a mut self, transl_table: u8) -> &'a mut Self {
		self.transl_table = transl_table;
		self
	}

	pub fn halt_at_stop(&'a mut self, to_stop: bool) -> &'a mut Self {
		self.to_stop = to_stop;
		self
	}

	pub fn stop_symbol(&'a mut self, stop_symbol: u8) -> &'a mut Self {
		self.stop_symbol = stop_symbol;
		self
	}

	pub fn run(&self) -> BioResult<Seq> {
		let mut out: Vec<u8> = Vec::with_capacity(self.seq.0.len());
		for codon in self.seq.0.chunks_exact(3) {
			let aa = super::translationtable::get_aa(self.transl_table, codon)?;
			if self.to_stop && aa == b'*' {
				break;
			}
			out.push(aa);
		}

		if self.stop_symbol != b'*' {
			for x in &mut out {
				if *x == b'*' {
					*x = self.stop_symbol;
				}
			}
		}

		Ok(Seq::from_bytes(&out))
	}
}

#[cfg(test)]
mod test {
	use crate::Seq;
	#[test]
	fn builder() {
		let dna = Seq::new("atgcagtcgctagctagctagaga");
		let out_1 = "MQSLAS-R";
		let out_2 = "MQSLAS";
		let out_3 = "MQSLAS*S";
		let res_1 = dna.translate().halt_at_stop(false).stop_symbol(b'-').run();
		let res_2 = dna.translate().halt_at_stop(true).run();
		let res_3 = dna.translate().transl_table(5).run();
		assert_eq!(out_1.to_string(), res_1.unwrap().to_string());
		assert_eq!(out_2.to_string(), res_2.unwrap().to_string());
		assert_eq!(out_3.to_string(), res_3.unwrap().to_string());
	}
}
