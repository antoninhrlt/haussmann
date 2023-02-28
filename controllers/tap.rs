// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{Widget, widgets, ToAny, DebugWidget, graphics::{Point, Size, colours::RGBA}};

use super::ControllerFn;

/// Whether a tap is over the tappable zone.
/// 
/// This function must be called after the widget is drawn, never before.
#[inline]
pub fn is_tapped(tap: Point, widget_position: Point, widget_size: Size) -> bool {
    let p: Point = widget_position;
    let s: Size = widget_size;

    let x = tap[0];
    let y = tap[1];

    x >= p[0] && x <= p[0] + s[0] as isize 
        && y >= p[1] && y <= p[1] + s[1] as isize
}

/// Controller detecting taps on a widget.
pub struct Detector<T: Widget> {
    /// The wrapped widget.
    pub widget: Box<T>,
    /// The zone covered by the tap detection.
    pub zone: (Point, Size),
    /// Function to call when the widget is tapped.
    tap: ControllerFn<T>,
}

impl<T: Widget> std::fmt::Debug for Detector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "tap::Detector")
    }
}

widgets::dynamic_controller!(Detector<T>);

impl<T: Widget + 'static> Widget for Detector<T> {
    fn build(&self) -> Box<dyn Widget> {
        self.widget.build()
    }

    fn colour(&self) -> RGBA {
        self.widget.colour()
    }
}

impl<T: Widget> Detector<T> {
    /// Creates a new tap detector.
    pub fn new(widget: T, tap: ControllerFn<T>) -> Self {
        Self {
            widget: Box::new(widget),
            zone: ([-1, -1], [0, 0]),
            tap,
        }
    }

    /// Function to call when the wrapped widget was tapped.
    pub fn on_tap(&mut self) {
        let tap = self.tap;
        tap(&mut self.widget);
    }
}
