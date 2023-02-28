// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Integrate highly customizable widgets and themes for any Rust application
//! or GUI library.
//!
//! ## Introduction
//! More than explaining how to use this crate, this documentation want to help
//! understand how the project is organized, and how the functions actually
//! work. Tutorials, examples and more can be found on the
//! [repository (GitHub)](https://github.com/antoninhrlt/haussmann).
//!
//! However, some examples can be found in this documentation when they are
//! useful to understand how the item works. Sometimes, a good example is better
//! than lines of documentation.
//!
//! This documentation is for people who want to contribute to the project but
//! also for people who just want to understand some concepts.
//!
//! ## Purpose
//! This project is not a GUI library itself, but it gives highly customizable
//! widgets and themes to be integrated in a GUI library or directly in an
//! application using a system API
//! ([windows-rs](https://github.com/microsoft/windows-rs),
//! [x11rb](https://github.com/psychon/x11rb), ...). Furthermore, the crate
//! contains tools to transform widgets into shapes and to place and align
//! widgets on a drawable surface. The [`Drawer`](graphics::Drawer) trait can be
//! implemented on a drawable surface to easily draw widgets on it.
//!
//! ## Organization
//! The crate is organized in different parts:
//! - Widgets (located in [`mod@widgets`])
//! - Controllers (located in [`controllers`]), they are also widgets but not
//! designed to be drawn but to wrap a widget and control stuff like events on
//! it.
//! - Utilities: simple structures, parameters for widgets... (located in
//! [`crate`])
//! - Graphics: drawer, aligner, shapes... (located in [`graphics`])
//!
//! ## Note
//! This project want to have the simplest usage possible. Indeed, imports are
//! simplified the most possible (well, it does not mean all elements can be
//! imported from the crate, but repetition is avoided and imports like
//! `crate::widgets::Widget` are replaced by `crate::Widget` for example) and
//! widgets have different constructors to avoid filling all the fields when
//! they are not all defined (check the widgets documentation).

#![warn(missing_docs)]

#[path = "../controllers/mod.rs"]
pub mod controllers;

#[path = "../widgets/mod.rs"]
pub mod widgets;

pub mod graphics;

mod align;
mod border;
mod direction;
mod font;
mod overflow;
mod radius;
mod theme;

pub use align::{Align, TextAlign};
pub use border::Border;
pub use direction::Direction;
pub use font::*;
pub use overflow::Overflow;
pub use radius::Radius;
pub use theme::{TextTheme, Theme};

pub use widgets::{DebugWidget, Widget};

/// Trait to implement on *any* object in order to be able to convert it as a
/// [`std::any::Any`] value.
pub trait ToAny {
    /// Converts the `self` value into a [`std::any::Any`] value.
    ///
    /// Generally implemented as following:
    /// ```
    /// impl ToAny for Foo {
    ///     fn as_any(&self) -> &dyn std::any::Any {
    ///         self
    ///     }    
    /// }
    /// ```
    fn as_any(&self) -> &dyn std::any::Any;

    /// Converts the `self` value into a [`std::any::Any`] mutable value.
    ///
    /// Generally implemented as following:
    /// ```
    /// impl ToAny for Foo {
    ///     fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    ///         self
    ///     }    
    /// }
    /// ```
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}
