// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Tools to draw, to align in layouts, to shape and to colour
//! [`Widget`s](crate::Widget).

mod aligner;
pub mod colours;
mod builder;
pub mod shapes;

pub use builder::ShapesBuilder;
pub use shapes::Shape;

/// Vector of type `T` with 2 values.
pub type Vec2<T> = [T; 2];

/// Point on a 2D surface of coordinates `[x, y]` being `isize` values.
pub type Point = Vec2<isize>;

/// Size of `[width, height]` being `usize` values.
pub type Size = Vec2<usize>;
