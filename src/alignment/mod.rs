pub mod hamming;
mod lcs;
pub mod overlap;
pub mod palindrome;
mod substring;
pub use lcs::longest_common_sequence;
pub use substring::{subsequence, substring};
pub mod protein_motif;
