use std::{collections::HashMap, ops::Index};

use crate::Seq;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BwtSeq(Vec<u8>);

impl BwtSeq {
	#[must_use]
	pub fn from(s: &Seq) -> Self {
		bwt(s)
	}

	#[must_use]
	pub fn inverse(&self) -> Seq {
		Seq(inverse_bwt(self))
	}

	#[must_use]
	pub fn len(&self) -> usize {
		self.0.len()
	}

	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

impl<T> Index<T> for BwtSeq
where
	T: std::slice::SliceIndex<[u8]>,
{
	type Output = T::Output;

	fn index(&self, index: T) -> &Self::Output {
		&self.0[index]
	}
}

#[must_use]
pub fn bwt(s: &Seq) -> BwtSeq {
	let mut s = s.0.clone();
	// s.insert(0, b'$');
	s.push(b'$');

	let mut rotations = Vec::with_capacity(s.len());
	for i in 0..s.len() {
		let mut fused: Vec<u8> = s[i..].to_vec();
		fused.extend_from_slice(&s[..i]);
		rotations.push(fused);
	}

	rotations.sort();
	BwtSeq(rotations.iter().map(|x| *x.last().unwrap()).collect())
}

#[must_use]
fn inverse_bwt(s: &BwtSeq) -> Vec<u8> {
	let occ_arr = get_occurance_array(&s.0);
	let mut out = Vec::with_capacity(s.len());
	let mut first_col = s.0.clone();
	first_col.sort_unstable();
	let n_occ_arr = get_n_occurance_array(&first_col);
	let mut index = 0;

	for _ in 0..s.len() {
		let c = s.0[index];
		out.push(c);
		let occ = occ_arr[index];
		index = n_occ_arr.get(&c).unwrap()[occ];
	}

	out.pop();
	out.reverse();
	out
}

fn get_occurance_array(s: &[u8]) -> Vec<usize> {
	let mut map = HashMap::new();
	let mut out = Vec::with_capacity(s.len());
	for i in s {
		let e = map.entry(i).or_insert(0);
		out.push(*e);
		*e += 1;
	}
	out
}

fn get_n_occurance_array(s: &[u8]) -> HashMap<u8, Vec<usize>> {
	let mut map = HashMap::new();
	for (i, c) in s.iter().enumerate() {
		let e: &mut Vec<usize> = map.entry(*c).or_default();
		e.push(i);
	}
	map
}

#[cfg(test)]
mod test {
	use super::{bwt, get_n_occurance_array, inverse_bwt, BwtSeq, Seq};
	#[test]
	fn test_bwt1() {
		let input = Seq::new("catata");
		let output = BwtSeq(b"attc$aa".to_vec());
		assert_eq!(bwt(&input), output);
	}
	#[test]
	fn test_bwt2() {
		let input = Seq::new("aggct");
		let output = BwtSeq(b"t$ggac".to_vec());
		assert_eq!(bwt(&input), output);
	}
	#[test]
	fn test_bwt3() {
		let input = Seq::new("abracadabra");
		let output = BwtSeq(b"ard$rcaaaabb".to_vec());
		assert_eq!(bwt(&input), output);
	}

	#[test]
	fn test_ibwt1() {
		let input = BwtSeq(b"t$ggac".to_vec());
		let output = b"aggct";
		assert_eq!(inverse_bwt(&input), output);
	}
	#[test]
	fn test_ibwt2() {
		let input = BwtSeq(b"touo$toitritwcnndb".to_vec());
		let output = br"introductiontobwt";
		assert_eq!(inverse_bwt(&input), output);
	}

	#[test]
	fn test_occ_array() {
		let input = vec![1, 2, 3, 1, 1, 3, 4];
		let output = vec![0, 0, 0, 1, 2, 1, 0];
		assert_eq!(super::get_occurance_array(&input), output);
	}

	#[test]
	fn test_n_occ_array() {
		let input = b"abcdcbaca";
		let res = get_n_occurance_array(input);
		assert_eq!(res.get(&b'a').unwrap()[1], 6);
		assert_eq!(res.get(&b'b').unwrap()[0], 1);
	}
}
