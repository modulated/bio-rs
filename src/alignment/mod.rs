pub mod hamming;
mod common;
pub mod overlap;
pub mod palindrome;
mod substring;
pub use common::{longest_common_sequence, shortest_common_supersequence};
pub use substring::{subsequence, substring};
pub mod protein_motif;
