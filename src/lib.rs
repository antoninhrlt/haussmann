// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

#[path = "../widgets/mod.rs"]
pub mod widgets;

pub mod graphics;

mod align;
mod border;
mod direction;
mod font;
mod integrator;
mod overflow;
mod radius;
mod theme;

pub use align::{ Align, TextAlign };
pub use border::Border;
pub use direction::Direction;
pub use font::*;
pub use integrator::Integrator;
pub use overflow::Overflow;
pub use radius::Radius;
pub use theme::Theme;

pub use widgets::{ Widget, DebugWidget };

pub trait ToAny {
    fn as_any(&self) -> &dyn std::any::Any;
}
