// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Rules about widget overflowing.
#[derive(Debug, Clone, PartialEq)]
pub enum Overflow {
    /// Does not hide the widgets out of the layout dimensions.
    Ignore,
    /// Hides the widgets out of the layout dimensions.
    Hide,
}
