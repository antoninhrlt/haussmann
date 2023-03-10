// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::controllers::Controller;
use crate::graphics::{Point, Size};
use crate::graphics::draw::{Drawable, self};
use crate::Zone;

use super::Layout;

/// Wraps a [`Layout`] and permit to build [`Drawable`]s from the widgets in.
/// 
/// Has a defined position and size in a drawable zone such as a window, a 
/// canvas...
#[derive(Debug)]
pub struct View {
    /// The zone contained by the view.
    zone: Zone,
    /// The layout for the view.
    pub layout: Layout,
}

/// Creates a new view like its [`new`](View::new) function.
/// 
/// The layout parameter is not named.
#[macro_export]
macro_rules! view {
    (position: $position:expr, size: $size:expr, $layout:expr $(,)?) => {
        View::new(
            ($position, $size).into(), 
            $layout
        )
    };
}

impl View {
    /// Creates a new view at a certain point with a specified size.
    pub fn new(zone: Zone, layout: Layout) -> Self {
        Self {
            zone,
            layout,
        }
    }

    /// Creates drawables for the widgets in the view's layout.
    pub fn build(&self) -> Vec<Drawable> {
        // Creates a builder.
        let mut builder = draw::Builder::new(self.zone);
        // Builds the widgets of the view's layout.
        builder.build_view(self);
        // Returns the created drawables.
        builder.drawables
    }

    /// Updates the zone of the view, and builds again.
    /// 
    /// There is no need to rebuild if the view is contained in the same zone.
    pub fn rebuild(&mut self, position: Point, size: Size) -> Vec<Drawable> {
        self.zone = (position, size).into();
        self.build()
    }

    fn controllers_in<T>(layout: &mut Layout, drawables: &Vec<Drawable>, callback: &impl Fn(&mut T), i: &mut usize) 
        where T: Controller + 'static,
    {
        for widget in &mut layout.widgets {
            for j in *i..drawables.len() - 2 {
                if drawables[j].parent_id == drawables[j + 1].parent_id {
                    *i += 1;
                    continue;
                }

                break
            }

            // Encounters a layout, calls itself but with the retrieved layout 
            // as parameter.
            if let Some(layout) = widget.as_any_mut().downcast_mut::<Layout>() {
                *i += 1;
                Self::controllers_in(layout, drawables, callback, i);
                continue;
            }

            // Encounters a controller, calls the callback.
            if let Some(controller) = widget.as_any_mut().downcast_mut::<T>() {
                // Update the controller's zone.
                controller.update(drawables[*i].zone);
                // Calls the callback giving the controller as mutable reference.
                callback(controller);
            }
        }
    }

    /// Browses all the widgets to find controllers and call the given callback
    /// when encountered.
    /// 
    /// Browses in the same order as drawables are created.
    pub fn controllers<T>(&mut self, drawables: &Vec<Drawable>, callback: impl Fn(&mut T)) 
        where T: Controller + 'static,
    {
        let mut i: usize = 1;
        Self::controllers_in(&mut self.layout, drawables, &callback, &mut i);
    }
}
