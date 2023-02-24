// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[allow(missing_docs)]
pub type TextAlign = Align;

/// Alignment rule.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Align {
    /// Aligns item in the center of its parent.
    Center,
    /// Aligns item on the left of its parent.
    Left,
    /// Aligns item on the right of its parent.
    Right,
    /// Aligns item at the top of its parent.
    Top,
    /// Aligns item in the bottom of its parent.
    Bottom
}
