mod bytes;
pub mod fasta;
mod seq;
pub use fasta::{FASTA, parse_string_to_vec_of_fasta};
pub use seq::Seq;
