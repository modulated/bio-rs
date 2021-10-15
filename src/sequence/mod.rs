mod bytes;
pub mod fasta;
mod seq;
pub use fasta::{parse_string_to_vec_of_fasta, FASTA};
pub use seq::Seq;
