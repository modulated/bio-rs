use crate::*;

#[derive(Debug, PartialEq)]
pub struct RNASequence {
    pub seq: Vec<RNA>
}

impl RNASequence {
    pub fn new(str: &str) -> Self {
        use std::convert::TryFrom;
        let mut vec: Vec<RNA> = Vec::new();

        for c in str.chars() {
            let dna = RNA::try_from(&c);
            if let Ok(i) = dna {
                vec.push(i);
            }            
        }

        RNASequence {
            seq: vec
        }
    }

    pub fn counts(&self) -> (u64, u64, u64, u64) {
        let mut out = (0u64, 0u64, 0u64, 0u64);

        for n in &self.seq {
            match n {
                RNA::A => out.0 += 1,
                RNA::C => out.1 += 1,
                RNA::G => out.2 += 1,
                RNA::U => out.3 += 1,
            }
        }

        out
    }

    pub fn transcribe(&self) -> DNASequence {
        let mut seq: Vec<DNA> = vec![];

        for c in &self.seq {
            seq.push(DNA::from(*c))
        }

        DNASequence {
            seq
        }
    }

    pub fn reverse_complement(&self) -> RNASequence {
        RNASequence {
            seq: self.seq.iter().rev().map(|c|c.complement()).collect()
        }
    }

    pub fn translate(&self) -> Protein {
        let mut vec = vec![];

        for n in self.seq.chunks(3) {
            if n.len() != 3 { continue; }
            let c = Codon::new(n[0], n[1], n[2]);
            let a: AminoAcid = (&c).into();
            vec.push(a);
        }

        Protein {
            seq: vec
        }
    }
}

impl std::fmt::Display for RNASequence {
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
    use crate::{RNA, RNASequence};

    #[test]
    fn new() {
        let input = "GAGA";
        let output = RNASequence{seq:vec![RNA::G, RNA::A, RNA::G, RNA::A]};
        assert_eq!(output, RNASequence::new(input));
    }

    #[test]
    fn to_string() {
        let input = "GAUGAUGC";
        let output = RNASequence::new(input).to_string();

        assert_eq!(input, output);
    }

    #[test]
    fn counts() {
        let input = "ACGUGAGCGAGUGAGU";
        let output = (4, 2, 7, 3);

        assert_eq!(output, RNASequence::new(input).counts());
    }

    #[test]
    fn transcribe() {
        let input = "AGCAUCAGUG";
        let output = "AGCATCAGTG";

        assert_eq!(output, RNASequence::new(input).transcribe().to_string());
    }

    #[test]
    fn reverse_complement() {
        let input = "ACUGA";
        let output = "UCAGU";

        assert_eq!(output, RNASequence::new(input).reverse_complement().to_string());
    }

    #[test]
    fn translate() {
        let input = "AUGGCCAUGGCGCCCAGAACUGAGAUCAAUAGUACCCGUAUUAACGGGUGA";
        let output = "MAMAPRTEINSTRING";

        assert_eq!(output, RNASequence::new(input).translate().to_string());
    }
}