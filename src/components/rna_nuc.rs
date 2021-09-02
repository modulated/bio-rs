use crate::DNA;
use super::NucleicAcid;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum RNA {
    A,
    C,
    G,
    U
}

impl NucleicAcid for RNA {
    fn complement(&self) -> Self {
        match self {
            RNA::A => RNA::U,
            RNA::C => RNA::G,
            RNA::G=> RNA::C,
            RNA::U => RNA::A
        }
    }
}

impl std::convert::TryFrom<&char> for RNA {
    type Error = ();
    fn try_from(item: &char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(RNA::A),
            'C' => Ok(RNA::C),
            'G' => Ok(RNA::G),
            'U' => Ok(RNA::U),
            _ => Err(())
        }
    }
}

impl From<DNA> for RNA {    
    fn from(item: DNA) -> Self {
        match item {
            DNA::A => RNA::A,
            DNA::C => RNA::C,
            DNA::G => RNA::G,
            DNA::T => RNA::U
        }
    }
}

impl From<&DNA> for RNA {    
    fn from(item: &DNA) -> Self {
        match item {
            DNA::A => RNA::A,
            DNA::C => RNA::C,
            DNA::G => RNA::G,
            DNA::T => RNA::U
        }
    }
}

impl std::fmt::Display for RNA {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RNA::A => write!(f, "A"),
            RNA::C => write!(f, "C"),
            RNA::G => write!(f, "G"),
            RNA::U => write!(f, "U")
        }   
    }
}


#[cfg(test)]
mod test {
    use crate::{NucleicAcid, RNA, DNA};
    use std::convert::TryFrom;

    #[test]
    fn complement() {
        let test_case = vec![RNA::A, RNA::C, RNA::G, RNA::U];
        let test_sol = vec![RNA::U, RNA::G, RNA::C, RNA::A];

        for (c,s) in test_case.iter().zip(test_sol.iter()) {
            assert_eq!(&c.complement(), s);
        }
    }

    #[test]
    fn conversion() {
        let test_case = vec!['A', 'C', 'G', 'U'];
        let test_sol = vec![RNA::A, RNA::C, RNA::G, RNA::U];

        for (c,s) in test_case.iter().zip(test_sol.iter()) {
            assert_eq!(&RNA::try_from(c).unwrap(), s);
        }
    }

    #[test]
    fn to_string() {
        assert_eq!("A", RNA::A.to_string());
    }

    #[test]
    fn from() {
        assert_eq!(DNA::from(RNA::U), DNA::T)
    }
}