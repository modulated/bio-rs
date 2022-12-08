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

#[inline]
pub(super) const fn transcribe_byte(byte: u8) -> u8 {
	match byte {
		b'U' => b'T',
		b'u' => b't',
		b'T' => b'U',
		b't' => b'u',
		_ => byte,
	}
}
