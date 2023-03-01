// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Controllers are wrappers for [`Widget`s](crate::Widget) having one or more 
//! function/s to call when specific one or more event/s happen/s.

pub mod tap;

use crate::{Widget, ToAny, DebugWidget, graphics::colours::RGBA};

/// Function to call when something happen on a widget.
type ControllerFn<T> = fn(widget: &mut T);

/// Wraps a widget to call functions when some events happen on it.
#[derive(Controller)]
pub struct Controller<T: Widget + 'static> {
    /// The controlled widget.
    pub widget: Box<T>,
    /// Function to call when the widget is tapped. 
    tap: Option<ControllerFn<T>>,
    /// Function to call when the widget is focused.
    focus: Option<ControllerFn<T>>,
    /// Function to call when the widget is unfocused.
    unfocus: Option<ControllerFn<T>>,
}

impl<T: Widget + 'static> Controller<T> {
    /// Creates a new controller for a widget.
    pub fn new(widget: T, tap: Option<ControllerFn<T>>, focus: Option<ControllerFn<T>>, unfocus: Option<ControllerFn<T>>) -> Self {
        Self {
            widget: Box::new(widget),
            tap,
            focus,
            unfocus,
        }
    }

    on!(tap, "tapped", on_tap); 
    on!(focus, "focused", on_focus);
    on!(unfocus, "unfocused", on_unfocus);
}

macro_rules! on {
    ($field:ident, $x:expr, $function:ident) => {
        #[allow(missing_docs)]
        pub fn $function(&mut self) -> Result<(), String> {
            match self.$field {
                Some(function) => Ok(function(&mut self.widget)), 
                None => Err(format!("undefined function when widget is {}", $x)),
            }
        }
    };
}

use haussmann_dev::Controller;
pub(crate) use on;
