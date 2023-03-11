// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    controllers::{Controller, ControllersBrowser},
    graphics::{draw::{Drawable, self}, Point, Size},
    Zone,
};

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

    /// Browses all the widgets to find controllers and call the given callback
    /// when encountered.
    /// 
    /// Browses in the same order as drawables are created. So, the zones of the 
    /// controllers are updated to the corresponding drawable.
    pub fn controllers<T>(&mut self, drawables: Vec<Drawable>, callback: impl Fn(&mut T)) 
    where 
        T: Controller + 'static,
    {
        let mut browser = ControllersBrowser::new(drawables, callback);
        browser.browse_layout(&mut self.layout);
    }
}
