// TODO: Fix generic to allow type DNASequence and RNASequence (Trait Sequence) to be passed direct as ref
pub fn distance<I: PartialEq, T: Clone + IntoIterator<Item = I>>(a: &T, b: &T) -> usize {
	let mut count = 0;
	for (i, j) in a.clone().into_iter().zip(b.clone().into_iter()) {
		if i != j {
			count += 1;
		}
	}

	count
}

#[cfg(test)]
mod test {
	use super::distance;
	use crate::Seq;

	#[test]
	fn hamming() {
		let a = Seq::new("GAGCCTACTAACGGGAT");
		let b = Seq::new("CATCGTAATGACGGCCT");

		assert_eq!(distance(&a.iter(), &b.iter()), 7);
	}
}
