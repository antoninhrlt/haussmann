// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    graphics::{
        colours::RGBA,
        Size,
    },
    DebugWidget, ToAny, Widget, widgets,
};

/// Wraps a widget giving it a fixed size.
#[derive(Debug, Widget)]
pub struct Container {
    /// The size of the zone.
    pub size: Size,
    /// The widget contained in this fixed zone.
    pub widget: Box<dyn Widget>,
}

impl Widget for Container {
    fn build(&self) -> Box<dyn Widget> {
        self.widget.build()
    }

    fn colour(&self) -> RGBA {
        self.widget.colour()
    }
}

impl Container {
    /// Creates a new containers.
    pub fn new<T: Widget + 'static>(
        size: Size,
        widget: T,
    ) -> Self {
        Self {
            size,
            widget: Box::new(widget),
        }
    }

    /// Creates the simple container possible.
    pub fn simple<T: Widget + 'static>(size: Size, widget: T) -> Self {
        Self {
            size,
            widget: Box::new(widget),
        }
    }
}