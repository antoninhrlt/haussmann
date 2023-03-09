// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Controllers are wrappers for [`Widget`s](crate::Widget) having one or more 
//! function/s to call when specific one or more event/s happen/s.

pub mod tap;

use crate::{Widget, graphics::{Point, Size}};

/// Function to call when something happen on a widget.
type ControllerFn<T> = fn(widget: &mut T);

/// Controllers wrap widgets in order to control what happen for them.
pub trait Controller: Widget {
    /// Returns the controlled zone which is the the position and size of the 
    /// drawable corresponding to the controlled widget.
    fn zone(&self) -> (Point, Size); 

    /// Updates the controlled zone.
    fn update(&mut self, zone: (Point, Size));
}
