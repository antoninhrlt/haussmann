// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::Layout, graphics::{Drawer, colours::RGBA}};

/// Helps integrate *haussmann* in any project.
pub trait Integrator: Drawer {
    /// Returns the layout containing all the widgets of the application.
    fn view() -> Layout;
    /// Runs the application.
    fn run(&mut self);
    /// Clears the drawable surface of the application.
    fn clear(&mut self, colour: RGBA);
}
