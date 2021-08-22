use crate::*;

#[derive(Debug, PartialEq)]
pub struct DNASequence {
    pub seq: Vec<DNA>
}

impl DNASequence {
    pub fn new(str: &str) -> Self {
        use std::convert::TryFrom;
        let mut vec: Vec<DNA> = Vec::new();

        for c in str.chars() {
            let dna = DNA::try_from(&c);
            if let Ok(i) = dna {
                vec.push(i);
            }            
        }

        DNASequence {
            seq: vec
        }
    }

    pub fn counts(&self) -> (u64, u64, u64, u64) {
        let mut out = (0u64, 0u64, 0u64, 0u64);

        for n in &self.seq {
            match n {
                DNA::A => out.0 += 1,
                DNA::C => out.1 += 1,
                DNA::G => out.2 += 1,
                DNA::T => out.3 += 1,
            }
        }

        out
    }

    pub fn transcribe(&self) -> RNASequence {
        let mut seq: Vec<RNA> = vec![];

        for c in &self.seq {
            seq.push(RNA::from(c))
        }

        RNASequence {
            seq
        }
    }

    pub fn reverse_complement(&self) -> DNASequence {
        DNASequence {
            seq: self.seq.iter().rev().map(|c|c.complement()).collect()
        }
    }
}

impl std::fmt::Display for DNASequence {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut res = String::new();

        for c in &self.seq {
            res += &c.to_string();
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod test {
    use crate::{DNASequence, DNA};

    #[test]
    fn new() {
        let input = "GAGA";
        let output = DNASequence{seq:vec![DNA::G, DNA::A, DNA::G, DNA::A]};
        assert_eq!(output, DNASequence::new(input));
    }

    #[test]
    fn to_string() {
        let input = "GATGATGC";
        let output = DNASequence::new(input).to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn counts() {
        let input = "ACGTGAGCGAGTGAG";
        let output = (4, 2, 7, 2);

        assert_eq!(output, DNASequence::new(input).counts());
    }

    #[test]
    fn transcribe() {
        let input = "AGCATCAGTG";
        let output = "AGCAUCAGUG";

        assert_eq!(output, DNASequence::new(input).transcribe().to_string());
    }

    #[test]
    fn reverse_complement() {
        let input = "ACTGA";
        let output = "TCAGT";

        assert_eq!(output, DNASequence::new(input).reverse_complement().to_string());
    }
}