// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Sides of a rectangle.
#[derive(Debug, Clone, PartialEq)]
pub enum Side {
    Bottom,
    Left,
    Right,
    Top,
}
