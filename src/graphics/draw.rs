// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to drawing of widgets.

use crate::{widgets::{Label, Image, Surface}, Widget};

use super::{Size, Point};

/// The object to draw from a [`Drawable`].
#[derive(Debug)]
pub enum Object {
    /// Wraps an [`Image`] in order to draw it.
    Image(Image),
    /// Wraps an [`Label`] in order to draw it.
    Label(Label),
    /// Wraps an [`Surface`] in order to draw it.
    Surface(Surface),
    /// Wraps a boxed dynamic [`Widget`] in order to draw it.
    Unknown(Box<dyn Widget>),
}

impl From<Image> for Object {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<Label> for Object {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<Surface> for Object {
    fn from(value: Surface) -> Self {
        Self::Surface(value)
    }
}

impl From<Box<dyn Widget>> for Object {
    fn from(value: Box<dyn Widget>) -> Self {
        Self::Unknown(value)
    }
}

/// Something that can be drawn and which has a defined position and size 
/// calculated at generation.
#[derive(Debug)]
pub struct Drawable {
    /// The object to be drawn.
    pub object: Object,
    /// One widget can create multiple drawables. Thanks to this identifier, 
    /// the parent (the creator of the drawable) is known.
    pub parent_id: i32,
    /// The position of the drawable.
    pub position: Point,
    /// The size of the drawable.
    pub size: Size,
}

impl Drawable {
    /// Creates a new drawable from an object at a defined position and size.
    /// 
    /// The parent has to be given here, it is not managed by the drawable 
    /// itself but created at drawables generation.
    pub fn new<T: Into<Object>>(object: T, position: Point, size: Size, parent: i32) -> Self {
        Self {
            object: object.into(),
            parent_id: parent,
            position,
            size,
        }
    }

    /// Returns the position and size of the drawable wrapped in a tuple.
    pub fn zone(&self) -> (Point, Size) {
        (self.position, self.size)
    }
}
