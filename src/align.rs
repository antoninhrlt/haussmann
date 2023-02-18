// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

pub type TextAlign = Align;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Align {
    Center,
    Left,
    Right,
    Top,
    Bottom
}
