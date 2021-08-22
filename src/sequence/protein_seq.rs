use crate::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub struct Protein {
    pub seq: Vec<AminoAcid>
}

impl Protein {
    pub fn new(str: &str) -> Self {
        use std::convert::TryFrom;
        let mut vec: Vec<AminoAcid> = Vec::new();

        for c in str.chars() {
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
            let count = out.entry(*n).or_insert(0u32);
            *count += 1;
        }

        out
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
    use crate::*;

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
}