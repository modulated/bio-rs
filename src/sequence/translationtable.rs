use crate::bioerror::BioError;

// Data from https://www.ncbi.nlm.nih.gov/Taxonomy/Utils/wprintgc.cgi?chapter=cgencodes#SG5
pub const AA_CODES: [&[u8; 64]; 34] = [
	b"________________________________________________________________", // Padding for logical integer offset
	b"FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 1: 1. The Standard Code
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSS**VVVVAAAADDEEGGGG", // 2. The Vertebrate Mitochondrial Code
	b"FFLLSSSSYY**CCWWTTTTPPPPHHQQRRRRIIMMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 3. The Yeast Mitochondrial Code
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 4. The Mold, Protozoan, and Coelenterate Mitochondrial Code and the Mycoplasma/Spiroplasma Code
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSSSSVVVVAAAADDEEGGGG", // 5. The Invertebrate Mitochondrial Code
	b"FFLLSSSSYYQQCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 6. The Ciliate, Dasycladacean and Hexamita Nuclear Code
	b"________________________________________________________________", // Padding
	b"________________________________________________________________", // Padding
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 9. The Echinoderm and Flatworm Mitochondrial Code
	b"FFLLSSSSYY**CCCWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 10. The Euplotid Nuclear Code
	b"FFLLSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 11. The Bacterial, Archaeal and Plant Plastid Code
	b"FFLLSSSSYY**CC*WLLLSPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 12. The Alternative Yeast Nuclear Code
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNKKSSGGVVVVAAAADDEEGGGG", // 13. The Ascidian Mitochondrial Code
	b"FFLLSSSSYYY*CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 14. The Alternative Flatworm Mitochondrial Code
	b"________________________________________________________________", // Padding
	b"FFLLSSSSYY*LCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 16. Chlorophycean Mitochondrial Code
	b"________________________________________________________________", // Padding
	b"________________________________________________________________", // Padding
	b"________________________________________________________________", // Padding
	b"________________________________________________________________", // Padding
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIMMTTTTNNNKSSSSVVVVAAAADDEEGGGG", // 21. Trematode Mitochondrial Code
	b"FFLLSS*SYY*LCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 22. Scenedesmus obliquus Mitochondrial Code
	b"FF*LSSSSYY**CC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 23. Thraustochytrium Mitochondrial Code
	b"FFLLSSSSYY**CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSSKVVVVAAAADDEEGGGG", // 24. Rhabdopleuridae Mitochondrial Code
	b"FFLLSSSSYY**CCGWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 25. Candidate Division SR1 and Gracilibacteria Code
	b"FFLLSSSSYY**CC*WLLLAPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 26. Pachysolen tannophilus Nuclear Code
	b"FFLLSSSSYYQQCCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 27. Karyorelict Nuclear Code
	b"FFLLSSSSYYQQCCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 28. Condylostoma Nuclear Code
	b"FFLLSSSSYYYYCC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 29. Mesodinium Nuclear Code
	b"FFLLSSSSYYEECC*WLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 30. Peritrich Nuclear Code
	b"FFLLSSSSYYEECCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSRRVVVVAAAADDEEGGGG", // 31. Blastocrithidia Nuclear Code
	b"________________________________________________________________", // Padding
	b"FFLLSSSSYYY*CCWWLLLLPPPPHHQQRRRRIIIMTTTTNNKKSSSKVVVVAAAADDEEGGGG", // 33. Cephalodiscidae Mitochondrial UAA-Tyr Code
];

pub fn get_aa(transl_table: u8, codon: &[u8]) -> Result<u8, BioError> {
	let code_idx = usize::from(
		(nuc_to_int(codon[0])? << 4) + (nuc_to_int(codon[1])? << 2) + nuc_to_int(codon[2])?,
	);
	Ok(AA_CODES[usize::from(transl_table)][code_idx])
}

const fn nuc_to_int(n: u8) -> Result<u8, BioError> {
	match n.to_ascii_uppercase() {
		b'T' | b'U' => Ok(0),
		b'C' => Ok(1),
		b'A' => Ok(2),
		b'G' => Ok(3),
		_ => Err(BioError::NotNucleotide(n)),
	}
}

#[cfg(test)]
mod test {
	use crate::bioerror::BioError;

	use super::get_aa;
	#[test]
	fn get_amino() {
		assert_eq!(get_aa(1, b"ATG").unwrap(), b'M');
		assert_eq!(get_aa(5, b"UGA").unwrap(), b'W');
		assert_eq!(
			get_aa(5, b"FGA").err().unwrap().to_string(),
			BioError::NotNucleotide(b'F').to_string()
		);
	}
}
