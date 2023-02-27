// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{
        colours::{self, RGBA},
        shapes, Shape, Size, Point,
    },
    Border, DebugWidget, ToAny, Widget, widgets,
};

/// Fixed zone containing a widget which is not able to go beyond it and which
/// will be of size `self.size`.
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

widgets::dynamic_widget!(Container);

impl Widget for Container {
    /// Returns a rectangle of size `self.size` filled with colour
    /// `self.colour`, with borders if defined.
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        assert_ne!(position, None);
        assert_eq!(size, None);

        shapes::Builder::new()
            .rectangle_at(position.unwrap(), self.size, self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Container {
    /// Creates a new containers.
    pub fn new<T: Widget + 'static>(
        size: Size,
        colour: RGBA,
        borders: [Border; 4],
        widget: T,
    ) -> Self {
        Self {
            size,
            colour,
            borders: Some(borders),
            widget: Box::new(widget),
        }
    }

    /// Creates the simple container possible.
    pub fn simple<T: Widget + 'static>(size: Size, widget: T) -> Self {
        Self {
            size,
            colour: colours::TRANSPARENT,
            borders: None,
            widget: Box::new(widget),
        }
    }

    /// Creates a container with a defined colour.
    pub fn coloured<T: Widget + 'static>(size: Size, colour: RGBA, widget: T) -> Self {
        Self {
            size,
            colour,
            borders: None,
            widget: Box::new(widget),
        }
    }

    /// Creates a container with borders.
    pub fn bordered<T: Widget + 'static>(size: Size, borders: [Border; 4], widget: T) -> Self {
        Self {
            size,
            colour: RGBA::default(),
            borders: Some(borders),
            widget: Box::new(widget),
        }
    }
}
