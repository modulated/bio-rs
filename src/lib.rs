#[deny(clippy::all)]
#[warn(clippy::cargo)]
#[warn(clippy::nursery)]
#[warn(clippy::pedantic)]
mod alignment;
// mod components;
pub mod combinatorics;
mod sequence;
pub mod util;
pub use alignment::*;
// pub use components::*;
pub use sequence::*;
