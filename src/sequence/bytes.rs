pub(super) fn codon_to_amino(codon: &[u8]) -> u8 {
	#[allow(unused_assignments)]
	let mut out: u8 = 0;
	match codon[0] {
		b'A' | b'a' => match codon[1] {
			b'A' | b'a' => match codon[2] {
				b'A' | b'a' => out = b'K',
				b'C' | b'c' => out = b'N',
				b'G' | b'g' => out = b'K',
				b'T' | b't' | b'u' | b'U' => out = b'N',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'C' | b'c' => match codon[2] {
				b'A' | b'a' => out = b'T',
				b'C' | b'c' => out = b'T',
				b'G' | b'g' => out = b'T',
				b'T' | b't' | b'u' | b'U' => out = b'T',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'G' | b'g' => match codon[2] {
				b'A' | b'a' => out = b'R',
				b'C' | b'c' => out = b'S',
				b'G' | b'g' => out = b'R',
				b'T' | b't' | b'u' | b'U' => out = b'S',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'T' | b't' | b'u' | b'U' => match codon[2] {
				b'A' | b'a' => out = b'I',
				b'C' | b'c' => out = b'I',
				b'G' | b'g' => out = b'M',
				b'T' | b't' | b'u' | b'U' => out = b'I',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			_ => panic!("Unexpected character in codon: {}", codon[1]),
		},
		b'C' | b'c' => match codon[1] {
			b'A' | b'a' => match codon[2] {
				b'A' | b'a' => out = b'Q',
				b'C' | b'c' => out = b'H',
				b'G' | b'g' => out = b'Q',
				b'T' | b't' | b'u' | b'U' => out = b'H',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'C' | b'c' => match codon[2] {
				b'A' | b'a' => out = b'P',
				b'C' | b'c' => out = b'P',
				b'G' | b'g' => out = b'P',
				b'T' | b't' | b'u' | b'U' => out = b'P',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'G' | b'g' => match codon[2] {
				b'A' | b'a' => out = b'R',
				b'C' | b'c' => out = b'R',
				b'G' | b'g' => out = b'R',
				b'T' | b't' | b'u' | b'U' => out = b'R',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'T' | b't' | b'u' | b'U' => match codon[2] {
				b'A' | b'a' => out = b'L',
				b'C' | b'c' => out = b'L',
				b'G' | b'g' => out = b'L',
				b'T' | b't' | b'u' | b'U' => out = b'L',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			_ => panic!("Unexpected character in codon: {}", codon[1]),
		},
		b'G' | b'g' => match codon[1] {
			b'A' | b'a' => match codon[2] {
				b'A' | b'a' => out = b'E',
				b'C' | b'c' => out = b'D',
				b'G' | b'g' => out = b'E',
				b'T' | b't' | b'u' | b'U' => out = b'D',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'C' | b'c' => match codon[2] {
				b'A' | b'a' => out = b'A',
				b'C' | b'c' => out = b'A',
				b'G' | b'g' => out = b'A',
				b'T' | b't' | b'u' | b'U' => out = b'A',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'G' | b'g' => match codon[2] {
				b'A' | b'a' => out = b'G',
				b'C' | b'c' => out = b'G',
				b'G' | b'g' => out = b'G',
				b'T' | b't' | b'u' | b'U' => out = b'G',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'T' | b't' | b'u' | b'U' => match codon[2] {
				b'A' | b'a' => out = b'V',
				b'C' | b'c' => out = b'V',
				b'G' | b'g' => out = b'V',
				b'T' | b't' | b'u' | b'U' => out = b'V',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			_ => unreachable!(format!("Unexpected character in codon: {}", codon[1])),
		},
		b'T' | b't' | b'u' | b'U' => match codon[1] {
			b'A' | b'a' => match codon[2] {
				b'A' | b'a' => out = b'*',
				b'C' | b'c' => out = b'Y',
				b'G' | b'g' => out = b'*',
				b'T' | b't' | b'u' | b'U' => out = b'Y',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'C' | b'c' => match codon[2] {
				b'A' | b'a' => out = b'S',
				b'C' | b'c' => out = b'S',
				b'G' | b'g' => out = b'S',
				b'T' | b't' | b'u' | b'U' => out = b'S',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'G' | b'g' => match codon[2] {
				b'A' | b'a' => out = b'*',
				b'C' | b'c' => out = b'C',
				b'G' | b'g' => out = b'W',
				b'T' | b't' | b'u' | b'U' => out = b'C',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			b'T' | b't' | b'u' | b'U' => match codon[2] {
				b'A' | b'a' => out = b'L',
				b'C' | b'c' => out = b'F',
				b'G' | b'g' => out = b'L',
				b'T' | b't' | b'u' | b'U' => out = b'F',
				_ => panic!("Unexpected character in codon: {}", codon[2]),
			},
			_ => panic!("Unexpected character in codon: {}", codon[1]),
		},
		_ => panic!("Unexpected character in codon: {}", codon[0]),
	};
	out
}

pub(super) fn complement_slice(bytes: &[u8]) -> Vec<u8> {
	let mut out = Vec::with_capacity(bytes.len());
	for c in bytes {
		out.push(complement_byte(*c));
	}
	out
}

#[inline]
pub(super) fn complement_byte(byte: u8) -> u8 {
	#[allow(unused_assignments)]
	let mut out = 0;
	match byte {
		b'A' | b'a' => out = b'T',
		b'C' | b'c' => out = b'G',
		b'G' | b'g' => out = b'C',
		b'T' | b't' | b'U' | b'u' => out = b'A',
		_ => panic!("Unexpected character {}", std::ascii::escape_default(byte)),
	}
	out
}

#[cfg(test)]
mod test {
	use super::codon_to_amino;

	#[test]
	fn test_codon_to_amino() {
		let input = vec![
			[b'U', b'A', b'C'],
			[b'U', b'G', b'A'],
			[b'A', b'A', b'G'],
			[b'T', b'T', b'T'],
			[b'T', b'G', b'G'],
			[b'A', b'T', b'G'],
		];
		let output = vec![b'Y', b'*', b'K', b'F', b'W', b'M'];

		for (i, o) in input.iter().zip(output) {
			assert_eq!(codon_to_amino(i), o);
		}
	}
}
