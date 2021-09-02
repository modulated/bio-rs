use crate::*;
use crate::Codon;
use super::NucleicAcid;

#[derive(PartialEq, Eq, core::hash::Hash, Debug, Clone, Copy)]
pub enum AminoAcid {
    Alanine,
    Arginine,
    Asparagine,
    Aspartate,
    Cysteine,
    Glutamate,
    Glutamine,
    Glycine,
    Histidine,
    Isoleucine,
    Leucine,
    Lysine,
    Methionine,
    Phenylalanine,
    Proline,
    Serine,
    Threonine,
    Tryptophan,
    Tyrosine,
    Valine,
    Stop(StopColor)
}

#[derive(PartialEq, Eq, core::hash::Hash, Debug, Clone, Copy)]
pub enum StopColor {
    Amber,
    Ochre,
    Opal
}

// AminoAcid.to_string()
impl std::fmt::Display for AminoAcid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AminoAcid::Alanine => write!(f, "A"),
            AminoAcid::Arginine => write!(f, "R"),
            AminoAcid::Asparagine => write!(f, "N"),
            AminoAcid::Aspartate => write!(f, "D"),
            AminoAcid::Cysteine => write!(f, "C"),
            AminoAcid::Glutamate => write!(f, "E"),
            AminoAcid::Glutamine => write!(f, "Q"),
            AminoAcid::Glycine => write!(f, "G"),
            AminoAcid::Histidine => write!(f, "H"),
            AminoAcid::Isoleucine => write!(f, "I"),
            AminoAcid::Leucine => write!(f, "L"),
            AminoAcid::Lysine => write!(f, "K"),
            AminoAcid::Methionine => write!(f, "M"),
            AminoAcid::Phenylalanine => write!(f, "F"),
            AminoAcid::Proline => write!(f, "P"),
            AminoAcid::Serine => write!(f, "S"),
            AminoAcid::Threonine => write!(f, "T"),
            AminoAcid::Tryptophan => write!(f, "W"),
            AminoAcid::Tyrosine => write!(f, "Y"),
            AminoAcid::Valine => write!(f, "V"),
            AminoAcid::Stop(_) => write!(f, ""),
        }   
    }
}

// AminoAcid <-> &char
use std::convert::TryFrom;
impl TryFrom<&char> for AminoAcid {
    type Error = ();
    fn try_from(item: &char) -> Result<Self, Self::Error> {
        match item {
            'A' => Ok(AminoAcid::Alanine),
            'R' => Ok(AminoAcid::Arginine),
            'N' => Ok(AminoAcid::Asparagine),
            'D' => Ok(AminoAcid::Aspartate),
            'C' => Ok(AminoAcid::Cysteine),
            'E' => Ok(AminoAcid::Glutamate),
            'Q' => Ok(AminoAcid::Glutamine),
            'G' => Ok(AminoAcid::Glycine),
            'H' => Ok(AminoAcid::Histidine),
            'I' => Ok(AminoAcid::Isoleucine),
            'L' => Ok(AminoAcid::Leucine),
            'K' => Ok(AminoAcid::Lysine),
            'M' => Ok(AminoAcid::Methionine),
            'F' => Ok(AminoAcid::Phenylalanine),
            'P' => Ok(AminoAcid::Proline),
            'S' => Ok(AminoAcid::Serine),
            'T' => Ok(AminoAcid::Threonine),
            'W' => Ok(AminoAcid::Tryptophan),
            'Y' => Ok(AminoAcid::Tyrosine),
            'V' => Ok(AminoAcid::Valine),
            _ => Err(())
        }
    }
}

impl<T: NucleicAcid + Into<DNA> + Copy> From<&Codon<T>> for AminoAcid {
    fn from(item: &Codon<T>) -> Self {
        match item.first.into() {
            DNA::A => {
                match item.second.into() {
                    DNA::A => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Lysine },
                            DNA::C => { AminoAcid::Asparagine },
                            DNA::G => { AminoAcid::Lysine },
                            DNA::T => { AminoAcid::Asparagine },
                        }
                    },
                    DNA::C => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Threonine },
                            DNA::C => { AminoAcid::Threonine },
                            DNA::G => { AminoAcid::Threonine },
                            DNA::T => { AminoAcid::Threonine },
                        }
                    },
                    DNA::G => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Arginine },
                            DNA::C => { AminoAcid::Serine },
                            DNA::G => { AminoAcid::Arginine },
                            DNA::T => { AminoAcid::Serine },
                        }
                    },
                    DNA::T => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Isoleucine },
                            DNA::C => { AminoAcid::Isoleucine },
                            DNA::G => { AminoAcid::Methionine },
                            DNA::T => { AminoAcid::Isoleucine },
                        }
                    },
                }
            },
            DNA::C => {
                match item.second.into() {
                    DNA::A => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Glutamine },
                            DNA::C => { AminoAcid::Histidine },
                            DNA::G => { AminoAcid::Glutamine },
                            DNA::T => { AminoAcid::Histidine },
                        }
                    },
                    DNA::C => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Proline },
                            DNA::C => { AminoAcid::Proline },
                            DNA::G => { AminoAcid::Proline },
                            DNA::T => { AminoAcid::Proline },
                        }
                    },
                    DNA::G => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Arginine },
                            DNA::C => { AminoAcid::Arginine },
                            DNA::G => { AminoAcid::Arginine },
                            DNA::T => { AminoAcid::Arginine },
                        }
                    },
                    DNA::T => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Leucine },
                            DNA::C => { AminoAcid::Leucine },
                            DNA::G => { AminoAcid::Leucine },
                            DNA::T => { AminoAcid::Leucine },
                        }
                    },
                }
            },
            DNA::G => {
                match item.second.into() {
                    DNA::A => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Glutamate },
                            DNA::C => { AminoAcid::Aspartate },
                            DNA::G => { AminoAcid::Glutamate },
                            DNA::T => { AminoAcid::Aspartate },
                        }
                    },
                    DNA::C => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Alanine },
                            DNA::C => { AminoAcid::Alanine },
                            DNA::G => { AminoAcid::Alanine },
                            DNA::T => { AminoAcid::Alanine },
                        }
                    },
                    DNA::G => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Glycine },
                            DNA::C => { AminoAcid::Glycine },
                            DNA::G => { AminoAcid::Glycine },
                            DNA::T => { AminoAcid::Glycine },
                        }
                    },
                    DNA::T => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Valine },
                            DNA::C => { AminoAcid::Valine },
                            DNA::G => { AminoAcid::Valine },
                            DNA::T => { AminoAcid::Valine },
                        }
                    },
                }
            },
            DNA::T => {
                match item.second.into() {
                    DNA::A => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Stop(StopColor::Ochre) },
                            DNA::C => { AminoAcid::Tyrosine },
                            DNA::G => { AminoAcid::Stop(StopColor::Amber) },
                            DNA::T => { AminoAcid::Tyrosine },
                        }
                    },
                    DNA::C => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Serine },
                            DNA::C => { AminoAcid::Serine },
                            DNA::G => { AminoAcid::Serine },
                            DNA::T => { AminoAcid::Serine },
                        }
                    },
                    DNA::G => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Stop(StopColor::Opal) },
                            DNA::C => { AminoAcid::Cysteine },
                            DNA::G => { AminoAcid::Tryptophan },
                            DNA::T => { AminoAcid::Cysteine },
                        }
                    },
                    DNA::T => {
                        match item.third.into() {
                            DNA::A => { AminoAcid::Leucine },
                            DNA::C => { AminoAcid::Phenylalanine },
                            DNA::G => { AminoAcid::Leucine },
                            DNA::T => { AminoAcid::Phenylalanine },
                        }
                    },
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn to_string() {
        assert_eq!(AminoAcid::Valine.to_string(), "V");
    }

    #[test]
    fn from_char() {
        use std::convert::TryFrom;
        assert_eq!(AminoAcid::Valine, AminoAcid::try_from(&'V').unwrap());
    }

    #[test]
    fn from_codon() {        
        let aa = AminoAcid::from(&Codon::new(DNA::G, DNA::T, DNA::C));
        assert_eq!(AminoAcid::Valine, aa);
    }
}