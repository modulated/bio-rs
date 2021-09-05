use crate::{AminoAcid, substring};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Protein {
    pub seq: Vec<AminoAcid>
}

impl Protein {
    pub fn new(str: &str) -> Self {

        let mut vec: Vec<AminoAcid> = Vec::new();

        for c in str.chars() {
            use std::convert::TryFrom;
            let aa = AminoAcid::try_from(&c);
            if let Ok(i) = aa {
                vec.push(i);
            }            
        }

        Protein {
            seq: vec
        }
    }

    pub fn counts(&self) -> HashMap<AminoAcid, u32> {
        let mut out = HashMap::new();

        for n in &self.seq {
            let count = out.entry(*n).or_insert(0_u32);
            *count += 1;
        }

        out
    }

    pub fn substring(&self, pattern: &Protein) -> Vec<usize> {
        substring(&pattern.seq, &self.seq)
    }

    pub fn cleave_start_stop(&mut self) -> Option<Vec<Protein>> {
        let start = self.seq.iter().position(|x|x == &AminoAcid::Methionine);
        match start {
            None => None,
            Some(i) => Some(Protein { seq: self.seq[i..].to_owned()})
        }
    }
}

impl std::fmt::Display for Protein {
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
    use crate::{Protein, AminoAcid};

    #[test]
    fn new() {
        let protein = Protein::new("AC");

        assert_eq!(protein, Protein { seq: vec![AminoAcid::Alanine, AminoAcid::Cysteine]});
    }

    #[test]
    fn counts() {
        let protein = Protein::new("AGCKAM");
        let counts = protein.counts();

        assert_eq!(*counts.get(&AminoAcid::Alanine).unwrap(), 2u32);
    }

    #[test]
    fn substring() {
        let a = Protein::new("AMGCKAMAM");
        let b = Protein::new("AM");
        let fail = Protein::new("AMBAMBJAAJFEAMAEGJ"); // longer than haystack - should return empty vec

        let res = a.substring(&b);
        let res_empty = a.substring(&fail);
        let empty: Vec<usize> = Vec::new();
        assert_eq!(vec![1, 6, 8], res);
        assert_eq!(empty, res_empty);
    }

    #[test]
    fn cleave_start_stop() {
        let input = "LIPKETMLLGSFRLIPKETLIQVAGSSPCNLS";
        let output = "MLLGSFRLIPKETLIQVAGSSPCNLS";

        assert_eq!(Protein::new(&input).cleave_start_stop().unwrap().to_string(), output);
    }
}