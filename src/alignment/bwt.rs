use crate::Seq;
#[must_use]
pub fn bwt(s: &Seq) -> Vec<u8> {
	let mut s = s.0.clone();
	s.insert(0, b'$');

	let mut rotations = vec![];
	for i in 0..s.len() {
		let mut fused: Vec<u8> = s[i..].to_vec();
		fused.extend_from_slice(&s[..i]);
		rotations.push(fused);
	}

	rotations.sort();
	let res: Vec<u8> = rotations.iter().map(|x| *x.last().unwrap()).collect();

	res
}

#[cfg(test)]
mod test {
	use super::{bwt, Seq};
	#[test]
	fn test_bwt1() {
		let input = Seq::new("catata");
		let output = vec![b'a', b't', b't', b'c', b'$', b'a', b'a'];
		assert_eq!(bwt(&input), output);
	}
	#[test]
	fn test_bwt2() {
		let input = Seq::new("aggct");
		let output = b"t$ggac";
		assert_eq!(bwt(&input), output);
	}
}
