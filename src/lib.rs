//! This crate is a (so far tiny) collection of iterator types
//! useful in implementing grid-based algorithms.
//!
//! Currently only [Bresenham's](struct.BresenhamIter.html) and
//! [Perimeter](struct.PerimeterIter.html) iterators are available.

mod iter;
mod coord;

pub use iter::{
    bresenham::BresenhamIter,
    perimeter::PerimeterIter
};

pub use coord::Coord;
