use super::NucleicAcid;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Codon<T: NucleicAcid> {
    pub first: T,
    pub second: T,
    pub third: T
}

impl<T: NucleicAcid> Codon<T> {
    pub fn new(first: T, second: T, third: T) -> Self {
        Codon {
            first,
            second,
            third
        }
    }
}

// Codon.to_string()
impl<T: NucleicAcid + std::fmt::Display> std::fmt::Display for Codon<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        write!(f, "{}{}{}", self.first, self.second, self.third)
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn new() {
        let c = Codon::new(DNA::A, DNA::A, DNA::A);
        assert_eq!(c, Codon { first: DNA::A, second: DNA::A, third: DNA::A});
    }

    #[test]
    fn to_string() {
        let c = Codon::new(DNA::A, DNA::A, DNA::A);
        assert_eq!(c.to_string(), "AAA");
    }
}