mod dna_seq;
pub use dna_seq::DNASequence;
mod rna_seq;
pub use rna_seq::RNASequence;

trait Sequence {
    fn complement(&self) -> Self;
    fn reverse(&self) -> Self;
    fn reverse_complement(&self) -> Self;
    fn to_string(&self) -> String;
}