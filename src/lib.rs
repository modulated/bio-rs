#[deny(clippy::all)]
#[warn(clippy::cargo)]
#[warn(clippy::nursery)]
// #[warn(clippy::pedantic)]
mod alignment;
pub mod combinatorics;
pub mod formats;
pub mod graph;
mod sequence;
pub mod util;
pub use alignment::*;
pub use sequence::*;
