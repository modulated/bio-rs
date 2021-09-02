use crate::{RNA, NucleicAcid};

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum DNA {
    A,
    C,
    G,
    T
}

impl NucleicAcid for DNA {
    fn complement(&self) -> Self {
        match self {
            DNA::A => DNA::T,
            DNA::C => DNA::G,
            DNA::G=> DNA::C,
            DNA::T => DNA::A
        }
    }
}

impl std::convert::TryFrom<&char> for DNA {
    type Error = ();
    fn try_from(item: &char) -> Result<Self, Self::Error> {
        return match item {
            'A' => Ok(DNA::A),
            'C' => Ok(DNA::C),
            'G' => Ok(DNA::G),
            'T' => Ok(DNA::T),
            _ => Err(())
        }
    }
}

impl From<RNA> for DNA {    
    fn from(item: RNA) -> Self {
        return match item {
            RNA::A => DNA::A,
            RNA::C => DNA::C,
            RNA::G => DNA::G,
            RNA::U => DNA::T
        }
    }
}

impl From<&RNA> for DNA {    
    fn from(item: &RNA) -> Self {
        return match item {
            RNA::A => DNA::A,
            RNA::C => DNA::C,
            RNA::G => DNA::G,
            RNA::U => DNA::T
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

    use crate::{DNA, RNA, NucleicAcid};

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