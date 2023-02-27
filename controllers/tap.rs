// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{Widget, widgets, ToAny, DebugWidget, graphics::{Point, Size, Shape}};

use super::ControllerFn;

/// Controller detecting taps on a widget.
pub struct Detector<T: Widget + 'static> {
    /// The wrapped widget.
    pub widget: Box<T>,
    /// Function to call when the widget is tapped.
    tap: ControllerFn<T>,
}

impl<T: Widget + 'static> std::fmt::Debug for Detector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tap::Detector")
    }
}

widgets::dynamic_controller!(Detector<T>);

impl<T: Widget> Widget for Detector<T> {
    fn shape(&self, position: Option<Point>, size: Option<Size>) -> Shape {
        self.widget.shape(position, size)
    }
}

impl<T: Widget> Detector<T> {
    /// Creates a new tap detector.
    pub fn new(widget: T, tap: ControllerFn<T>) -> Self {
        Self {
            widget: Box::new(widget),
            tap,
        }
    }

    /// Function to call when the wrapped widget was tapped.
    pub fn on_tap(&mut self) {
        let tap = self.tap;
        tap(&mut self.widget);
    }
}
