use crate::*;

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
            _ => return Err(())
        }
    }
}

use crate::Codon;
impl From<&Codon> for AminoAcid {
    fn from(item: &Codon) -> Self {
        match item.first {
            DNA::A => {
                match item.second {
                    DNA::A => {
                        match item.third {
                            DNA::A => { return AminoAcid::Lysine; },
                            DNA::C => { return AminoAcid::Asparagine; },
                            DNA::G => { return AminoAcid::Lysine; },
                            DNA::T => { return AminoAcid::Asparagine; },
                        }
                    },
                    DNA::C => {
                        match item.third {
                            DNA::A => { return AminoAcid::Threonine; },
                            DNA::C => { return AminoAcid::Threonine; },
                            DNA::G => { return AminoAcid::Threonine; },
                            DNA::T => { return AminoAcid::Threonine; },
                        }
                    },
                    DNA::G => {
                        match item.third {
                            DNA::A => { return AminoAcid::Arginine; },
                            DNA::C => { return AminoAcid::Serine; },
                            DNA::G => { return AminoAcid::Arginine; },
                            DNA::T => { return AminoAcid::Serine; },
                        }
                    },
                    DNA::T => {
                        match item.third {
                            DNA::A => { return AminoAcid::Isoleucine; },
                            DNA::C => { return AminoAcid::Isoleucine; },
                            DNA::G => { return AminoAcid::Methionine; },
                            DNA::T => { return AminoAcid::Isoleucine; },
                        }
                    },
                }
            },
            DNA::C => {
                match item.second {
                    DNA::A => {
                        match item.third {
                            DNA::A => { return AminoAcid::Glutamine; },
                            DNA::C => { return AminoAcid::Histidine; },
                            DNA::G => { return AminoAcid::Glutamine; },
                            DNA::T => { return AminoAcid::Histidine; },
                        }
                    },
                    DNA::C => {
                        match item.third {
                            DNA::A => { return AminoAcid::Proline; },
                            DNA::C => { return AminoAcid::Proline; },
                            DNA::G => { return AminoAcid::Proline; },
                            DNA::T => { return AminoAcid::Proline; },
                        }
                    },
                    DNA::G => {
                        match item.third {
                            DNA::A => { return AminoAcid::Arginine; },
                            DNA::C => { return AminoAcid::Arginine; },
                            DNA::G => { return AminoAcid::Arginine; },
                            DNA::T => { return AminoAcid::Arginine; },
                        }
                    },
                    DNA::T => {
                        match item.third {
                            DNA::A => { return AminoAcid::Leucine; },
                            DNA::C => { return AminoAcid::Leucine; },
                            DNA::G => { return AminoAcid::Leucine; },
                            DNA::T => { return AminoAcid::Leucine; },
                        }
                    },
                }
            },
            DNA::G => {
                match item.second {
                    DNA::A => {
                        match item.third {
                            DNA::A => { return AminoAcid::Glutamate; },
                            DNA::C => { return AminoAcid::Aspartate; },
                            DNA::G => { return AminoAcid::Glutamate; },
                            DNA::T => { return AminoAcid::Aspartate; },
                        }
                    },
                    DNA::C => {
                        match item.third {
                            DNA::A => { return AminoAcid::Alanine; },
                            DNA::C => { return AminoAcid::Alanine; },
                            DNA::G => { return AminoAcid::Alanine; },
                            DNA::T => { return AminoAcid::Alanine; },
                        }
                    },
                    DNA::G => {
                        match item.third {
                            DNA::A => { return AminoAcid::Glycine; },
                            DNA::C => { return AminoAcid::Glycine; },
                            DNA::G => { return AminoAcid::Glycine; },
                            DNA::T => { return AminoAcid::Glycine; },
                        }
                    },
                    DNA::T => {
                        match item.third {
                            DNA::A => { return AminoAcid::Valine; },
                            DNA::C => { return AminoAcid::Valine; },
                            DNA::G => { return AminoAcid::Valine; },
                            DNA::T => { return AminoAcid::Valine; },
                        }
                    },
                }
            },
            DNA::T => {
                match item.second {
                    DNA::A => {
                        match item.third {
                            DNA::A => { return AminoAcid::Stop(StopColor::Ochre); },
                            DNA::C => { return AminoAcid::Tyrosine; },
                            DNA::G => { return AminoAcid::Stop(StopColor::Amber); },
                            DNA::T => { return AminoAcid::Tyrosine; },
                        }
                    },
                    DNA::C => {
                        match item.third {
                            DNA::A => { return AminoAcid::Serine; },
                            DNA::C => { return AminoAcid::Serine; },
                            DNA::G => { return AminoAcid::Serine; },
                            DNA::T => { return AminoAcid::Serine; },
                        }
                    },
                    DNA::G => {
                        match item.third {
                            DNA::A => { return AminoAcid::Stop(StopColor::Opal); },
                            DNA::C => { return AminoAcid::Cysteine; },
                            DNA::G => { return AminoAcid::Tryptophan; },
                            DNA::T => { return AminoAcid::Cysteine; },
                        }
                    },
                    DNA::T => {
                        match item.third {
                            DNA::A => { return AminoAcid::Leucine; },
                            DNA::C => { return AminoAcid::Phenylalanine; },
                            DNA::G => { return AminoAcid::Leucine; },
                            DNA::T => { return AminoAcid::Phenylalanine; },
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