use crate::DNA;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Codon {
    pub first: DNA,
    pub second: DNA,
    pub third: DNA
}

impl Codon {
    pub fn new(first: DNA, second: DNA, third: DNA) -> Self {
        Codon {
            first,
            second,
            third
        }
    }
}

impl std::fmt::Display for Codon {
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