// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{graphics::{Size, Point, Shape, shapes, colours::{RGBA, self}}, Widget, DebugWidget, ToAny, Border};

/// Fixed zone containing a widget which is not able to go beyond it.
#[derive(Debug)]
pub struct Container {
    /// The size of the zone.
    pub size: Size,
    /// The colour of the container.
    pub colour: RGBA,
    /// Borders of the container.
    pub borders: Option<[Border; 4]>,
    /// The widget contained in this fixed zone.
    pub widget: Box<dyn Widget>,
}

crate::dynamic_widget!(Container);

impl Widget for Container {
    /// Returns a rectangle of size `self.size` filled with colour 
    /// `self.colour`, with borders if defined.
    fn shape(&self, size: Option<Size>) -> Shape {
        assert_eq!(size, None);

        shapes::Builder::new()
            .rectangle(self.size, self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Container {
    pub fn new<T: Widget + 'static>(size: Size, colour: RGBA, borders: [Border; 4], widget: T) -> Self {
        Self {
            size,
            colour,
            borders: Some(borders),
            widget: Box::new(widget),
        }
    }

    pub fn simple<T: Widget + 'static>(size: Size, widget: T) -> Self {
        Self {
            size,
            colour: colours::TRANSPARENT,
            borders: None,
            widget: Box::new(widget),
        }
    }

    pub fn coloured<T: Widget + 'static>(size: Size, colour: RGBA, widget: T) -> Self {
        Self {
            size,
            colour,
            borders: None,
            widget: Box::new(widget),
        }
    }
    
    pub fn bordered<T: Widget + 'static>(size: Size, borders: [Border; 4], widget: T) -> Self {
        Self {
            size,
            colour: RGBA::default(),
            borders: Some(borders),
            widget: Box::new(widget),
        }
    }
}
