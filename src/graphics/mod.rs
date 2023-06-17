// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Tools to draw, to align in layouts, to shape and to colour
//! [`Widget`s](crate::Widget).

use linbra::vector::Vector;

mod aligner;
pub mod colours;
pub mod draw;
mod sizer;

pub use aligner::Aligner;
pub use sizer::Sizer;

/// Point on a 2D surface of coordinates `[x, y]` being `isize` values.
pub type Point = Vector<isize, 2>;

/// Size of `[width, height]` being `usize` values.
pub type Size = Vector<usize, 2>;
