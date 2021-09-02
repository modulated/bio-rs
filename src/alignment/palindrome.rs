use crate::{NucleicAcid};

pub fn reverse_palindrome<T: PartialEq<T> + NucleicAcid>(string: &[T], min: usize, max: usize) -> Vec<(usize, usize)> {

    let mut out: Vec<(usize, usize)> = vec![];

    for i in min/2 ..(string.len() - min/2 + 1) {
        for j in 1 ..=max/2 {
            if string[i - j] != string[i - 1 + j].complement() {
                break;
            }
            if j >= min/2 {
                out.push((i-j+1, j*2));
            }
            if j + i + 1 > string.len() || j + 1 > i {
                break;
            }
        }
    }

    out
}

#[cfg(test)]
mod test {
    use super::reverse_palindrome;
    use crate::DNASequence;

    #[test]
    fn test_rev_palindrome() {
        let input = "TCAATGCATGCGGGTCTATATGCAT";
        let output = vec![(4, 6), (5, 4), (6, 6), (7, 4), (17, 4), (18, 4), (20, 6), (21, 4)];

        let mut res = reverse_palindrome(&DNASequence::new(input).seq, 4, 12);
        res.sort();
        assert_eq!(res, output);
    }

    #[test]
    fn test_rev_palindrome_2() {
        let input = "TATATA";
        let output = vec![(1, 4), (1, 6), (2, 4), (3, 4)];

        let mut res = reverse_palindrome(&DNASequence::new(input).seq, 4, 12);
        res.sort();
        assert_eq!(res, output);
    }

    #[test]
    fn test_rev_palindrome_3() {
        let input = "TTTAAATTTAAA ";
        let output = vec![(1, 6), (1, 12), (2, 4), (2, 10), (3, 8), (4, 6), (5, 4), (7, 6), (8, 4)];

        let mut res = reverse_palindrome(&DNASequence::new(input).seq, 4, 12);
        res.sort();
        assert_eq!(res, output);
    }
}