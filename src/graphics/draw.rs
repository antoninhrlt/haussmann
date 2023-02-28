// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Label, Image, Surface}, Widget};

use super::{Size, Point};

#[derive(Debug)]
pub enum Drawable {
    Image(Image, Point, Size),
    Label(Label, Point, Size),
    Surface(Surface, Point, Size),
    
    Unknown(Box<dyn Widget>, Point, Size),
}
