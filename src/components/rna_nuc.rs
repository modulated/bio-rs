use crate::DNA;
use super::NucleicAcid;

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
pub enum RNA {
    A,
    C,
    G,
    U
}

impl RNA {
    #[allow(dead_code)]
    pub fn complement(&self) -> RNA {
        match self {
            RNA::A => RNA::U,
            RNA::C => RNA::G,
            RNA::G=> RNA::C,
            RNA::U => RNA::A
        }
    }
}

impl NucleicAcid for RNA {}

impl std::convert::TryFrom<&char> for RNA {
    type Error = ();
    fn try_from(item: &char) -> Result<Self, Self::Error> {
        match item {
            'A' => return Ok(RNA::A),
            'C' => return Ok(RNA::C),
            'G' => return Ok(RNA::G),
            'U' => return Ok(RNA::U),
            _ => return Err(())
        }
    }
}

impl From<DNA> for RNA {    
    fn from(item: DNA) -> Self {
        match item {
            DNA::A => return RNA::A,
            DNA::C => return RNA::C,
            DNA::G => return RNA::G,
            DNA::T => return RNA::U           
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
    use std::convert::TryFrom;

    use super::{RNA, DNA};

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