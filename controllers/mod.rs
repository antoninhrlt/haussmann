// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Controllers are wrappers for [`Widget`s](crate::Widget) having one or more 
//! function/s to call when specific one or more event/s happen/s.

pub mod tap;

use std::marker::PhantomData;

use crate::{
    graphics::draw::{Drawable, DrawableAt}, 
    widgets::Layout,
    Widget,
    Zone,
};

/// Function to call when something happen on a widget.
type ControllerFn<T> = fn(widget: &mut T);

/// Controllers wrap widgets in order to control what happen for them.
pub trait Controller: Widget {
    /// Returns the controlled zone which is the the position and size of the 
    /// drawable corresponding to the controlled widget.
    fn zone(&self) -> &Zone; 

    /// Updates the controlled zone.
    fn update(&mut self, zone: Zone);
}

/// Browses widgets in order to find controllers. When they are found, calls the 
/// specified callback referencing the found controller.
/// 
/// All the zones of the controllers are updated following the drawable they 
/// correspond to.
pub(crate) struct ControllersBrowser<T: Controller, C: Fn(&mut T)> {
    /// Helps get the right drawable to update the current found controller.
    i: usize,
    drawables: Vec<Drawable>,
    /// The function to call when a controller is found.
    callback: C,
    /// Rust does not allow not using generic types.
    phantom: PhantomData<T>
}

impl<T: Controller, C: Fn(&mut T)> ControllersBrowser<T, C> {
    /// Creates a new browser for controllers.
    pub(crate) fn new(drawables: Vec<Drawable>, callback: C) -> Self {
        Self {
            i: 1,
            drawables,
            callback,
            phantom: PhantomData,
        }
    }
}

impl<T: Controller + 'static, C: Fn(&mut T)> ControllersBrowser<T, C> {
    /// Calls the callback with every controller found in the layout and 
    /// sub-layouts.
    pub(crate) fn browse_layout(&mut self, layout: &mut Layout) {
        for widget in &mut layout.widgets {
            // Encounters a layout, calls itself but with the retrieved layout 
            // as parameter.
            if let Some(layout) = widget.as_any_mut().downcast_mut::<Layout>() {
                self.i += 1;
                self.browse_layout(layout);
                continue;
            }

            // Encounters a controller, calls the callback.
            if let Some(controller) = widget.as_any_mut().downcast_mut::<T>() {
                // Update the controller's zone.
                controller.update(self.drawables.at(self.i).unwrap().zone);
                // Calls the callback giving the controller as mutable reference.
                let callback = &self.callback;
                callback(controller);
            }

            self.i += 1;
        }
    }
}
