use crate::RNA;
use super::NucleicAcid;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum DNA {
    A,
    C,
    G,
    T
}

impl DNA {
    #[allow(dead_code)]
    pub fn complement(&self) -> DNA {
        match self {
            DNA::A => DNA::T,
            DNA::C => DNA::G,
            DNA::G=> DNA::C,
            DNA::T => DNA::A
        }
    }
}

impl NucleicAcid for DNA {}

impl std::convert::TryFrom<&char> for DNA {
    type Error = ();
    fn try_from(item: &char) -> Result<Self, Self::Error> {
        match item {
            'A' => return Ok(DNA::A),
            'C' => return Ok(DNA::C),
            'G' => return Ok(DNA::G),
            'T' => return Ok(DNA::T),
            _ => return Err(())
        }
    }
}

impl From<RNA> for DNA {    
    fn from(item: RNA) -> Self {
        match item {
            RNA::A => return DNA::A,
            RNA::C => return DNA::C,
            RNA::G => return DNA::G,
            RNA::U => return DNA::T            
        }
    }
}

impl From<&RNA> for DNA {    
    fn from(item: &RNA) -> Self {
        match item {
            RNA::A => return DNA::A,
            RNA::C => return DNA::C,
            RNA::G => return DNA::G,
            RNA::U => return DNA::T            
        }
    }
}

impl std::fmt::Display for DNA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DNA::A => write!(f, "A"),
            DNA::C => write!(f, "C"),
            DNA::G => write!(f, "G"),
            DNA::T => write!(f, "T")
        }   
    }
}

#[cfg(test)]
mod test {
    use std::convert::TryFrom;

    use super::{DNA, RNA};

    #[test]
    fn complement() {
        let test_case = vec![DNA::A, DNA::C, DNA::G, DNA::T];
        let test_sol = vec![DNA::T, DNA::G, DNA::C, DNA::A];

        for (c,s) in test_case.iter().zip(test_sol.iter()) {
            assert_eq!(&c.complement(), s);
        }
    }

    #[test]
    fn conversion() {
        let test_case = vec!['A', 'C', 'G', 'T'];
        let test_sol = vec![DNA::A, DNA::C, DNA::G, DNA::T];

        for (c,s) in test_case.iter().zip(test_sol.iter()) {
            assert_eq!(&DNA::try_from(c).unwrap(), s);
        }
    }

    #[test]
    fn from() {
        assert_eq!(RNA::from(DNA::T), RNA::U);
    }
}