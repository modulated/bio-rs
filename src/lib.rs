#[deny(clippy::all)]
#[warn(clippy::cargo)]
#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
mod alignment;
pub mod combinatorics;
mod sequence;
pub mod util;
pub use alignment::*;
pub use sequence::*;
