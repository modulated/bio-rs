mod dna_seq;
mod rna_seq;
mod protein_seq;
pub mod fasta;
pub use dna_seq::DNASequence;
pub use rna_seq::RNASequence;
pub use fasta::FASTA;
pub use protein_seq::Protein;

pub trait Sequence: PartialEq {

    // fn complement(&self) -> Self;
    // fn reverse(&self) -> Self;
    // fn reverse_complement(&self) -> Self;
    // fn to_string(&self) -> String;
}

pub struct Seq { data: String }

impl Seq {
    pub fn new<T: Into<String>>(string: T) -> Self {
        Seq { data: string.into() }
    }

    pub fn counts(&self) -> (u32, u32, u32, u32) {
        let mut out = (0,0,0,0);
        for c in self.data.chars() {
            match c {
                'A' | 'a' => out.0 += 1,
                'C' | 'c' => out.1 += 1,
                'G' | 'g' => out.2 += 1,
                'T' | 't' => out.3 += 1,
                _ => {}
            }
        }

        out
    }
}