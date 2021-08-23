// TODO: Fix generic to allow type DNASequence and RNASequence (Trait Sequence) to be passed direct as ref
pub fn hamming_distance<I: PartialEq, T: Clone + IntoIterator<Item = I>>(a: &T, b: &T) -> usize {
    let mut count = 0;
    for (i,j) in a.clone().into_iter().zip(b.clone().into_iter()) {
        if i != j {
            count += 1;
        }
    }
    
    count
}

#[cfg(test)]
mod test {
    use crate::DNASequence;
    use super::hamming_distance;

    #[test]
    fn hamming() {
        let a = DNASequence::new("GAGCCTACTAACGGGAT");
        let b = DNASequence::new("CATCGTAATGACGGCCT");

        assert_eq!(hamming_distance(&a.seq, &b.seq), 7);
    }
}