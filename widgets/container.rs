// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use any::ToAny;

use haussmann_dev::Widget;

use crate::{
    graphics::Size,
    DebugWidget, Widget, themes::{Theme, Style},
};

/// Wraps a widget giving it a fixed size.
#[derive(Debug, Widget)]
pub struct Container {
    /// The size of the zone.
    pub size: Size,
    /// The widget contained in this fixed zone.
    pub widget: Box<dyn Widget>,
}

/// Creates a new container like its [`new`](Container::new) function.
#[macro_export]
macro_rules! container {
    (size: $size:expr, widget: $widget:expr $(,)?) => {
        Container::new($size, $widget)
    };
}

impl Widget for Container {
    fn build(&self) -> Box<dyn Widget> {
        self.widget.build()
    }

    fn style(&self, theme: &Theme) -> Style {
        self.widget.style(theme)
    }

    fn style_mut(&mut self, theme: &Theme) -> &mut Style {
        self.widget.style_mut(theme)
    }
}

impl Container {
    /// Creates a new containers
    pub fn new<T: Widget + 'static>(
        size: Size,
        widget: T,
    ) -> Self {
        Self {
            size,
            widget: Box::new(widget),
        }
    }
}
