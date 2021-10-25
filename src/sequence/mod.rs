mod bytes;
mod mutation;
mod seq;
pub use mutation::{compare_strings, transition_transversion_ratio, Mutation};
pub use seq::Seq;
