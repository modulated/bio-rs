mod dna_seq;
mod rna_seq;
pub mod fasta;
pub use dna_seq::DNASequence;
pub use rna_seq::RNASequence;
pub use fasta::FASTA;

trait Sequence {
    fn complement(&self) -> Self;
    fn reverse(&self) -> Self;
    fn reverse_complement(&self) -> Self;
    fn to_string(&self) -> String;
}