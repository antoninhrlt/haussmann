// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to tapping widgets.

use haussmann_dev::Controller;

use crate::{
    graphics::{Point, Size},
    themes::{Theme, Style},
    DebugWidget, 
    ToAny, 
    Widget, 
    Zone
};

use super::{Controller, ControllerFn};

/// Whether a tap is over the tappable zone.
/// 
/// This function must be called after the widget is drawn, never before.
#[inline]
pub fn is_tapped(tap: Point, zone: Zone) -> bool {
    let (x, y) = (tap[0], tap[1]);

    x >= zone.x() && x <= zone.x() + zone.width() as isize 
        && y >= zone.y() && y <= zone.y() + zone.height() as isize
}

/// Controller detecting taps on a widget.
#[derive(Controller)]
pub struct Detector<T: Widget> {
    /// The wrapped widget.
    pub widget: Box<T>,
    /// The zone covered by the tap detection.
    pub zone: Zone,
    /// Function to call when the widget is tapped.
    tap: ControllerFn<T>,
}

impl<T: Widget> Detector<T> {
    /// Creates a new tap detector.
    pub fn new(widget: T, tap: ControllerFn<T>) -> Self {
        Self {
            widget: Box::new(widget),
            zone: Zone {
                position: Point::from([0, 0]),
                size: Size::from([0, 0]),
            },
            tap,
        }
    }

    /// Function to call when the wrapped widget was tapped.
    pub fn on_tap(&mut self, theme: &Theme) {
        let tap = self.tap;
        tap(&mut self.widget, theme);
    }
}
