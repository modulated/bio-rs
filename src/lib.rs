#![deny(clippy::all)]
#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

mod alignment;
pub mod combinatorics;
pub mod formats;
pub mod graph;
mod sequence;
pub mod util;
pub use alignment::*;
pub use sequence::*;
pub use util::sets;
