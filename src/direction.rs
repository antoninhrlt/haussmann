// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

/// Direction rule for a [`Layout`](crate::widgets::Layout).
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Direction {
    /// The items are placed in row.
    Row,
    /// The items are placed in column.
    Column,
}
