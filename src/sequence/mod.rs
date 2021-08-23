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