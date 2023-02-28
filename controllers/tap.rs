// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use crate::{Widget, widgets, ToAny, DebugWidget, graphics::{Point, Size, Shape, colours::RGBA}};

use super::ControllerFn;

/// Controller detecting taps on a widget.
pub struct Detector<T: Widget> {
    /// The wrapped widget.
    pub widget: Box<T>,
    /// The zone which is equal to the position and the size of the widget after
    /// drawing.
    pub zone: Option<(Point, Size)>,
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
            zone: None,
            tap,
        }
    }

    /// Whether a tap is over the tappable zone.
    /// 
    /// This function must be called after the widget is drawn, never before.
    pub fn is_tapped(&self, tap: Point) -> bool {
        let p: Point = self.zone.unwrap().0;
        let s: Size = self.zone.unwrap().1;

        let x = tap[0];
        let y = tap[1];

        x >= p[0] && x <= p[0] + s[0] as isize 
            && y >= p[1] && y <= p[1] + s[1] as isize
    }

    /// Function to call when the wrapped widget was tapped.
    pub fn on_tap(&mut self) {
        let tap = self.tap;
        tap(&mut self.widget);
    }
}
