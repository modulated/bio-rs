mod bytes;
mod mutation;
mod seq;
mod translationbuilder;
mod translationtable;
pub use mutation::{compare_strings, transition_transversion_ratio, Mutation};
pub use seq::Seq;
pub use translationbuilder::TranslationBuilder;
