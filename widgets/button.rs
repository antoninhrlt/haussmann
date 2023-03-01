// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use super::{DebugWidget, Label, Widget, Layout};
use crate::{
    graphics::colours::RGBA,
    Border, Radius, ToAny, widgets, Overflow, Align, Direction,
};

/// Button widget with a label inside.
#[derive(Debug, Clone, Widget)]
pub struct Button {
    /// The colour of the button.
    pub colour: RGBA,
    /// The radius of the button.
    pub radius: Radius,
    /// The borders of the button.
    pub borders: Option<[Border; 4]>,
    /// The label in the center of the button.
    pub label: Label,
}

/// Creates a button controller to detect taps on it.
/// 
/// The "on tap" event closure is not necessary. In this case, it's not a button
/// controller which is created but a basic button.
#[macro_export]
macro_rules! button {
    (colour: $colour:expr, label: $label:expr $(,)?) => {
        Button::simple($colour, $label)
    };

    (colour: $colour:expr, label: $label:expr, on_tap: |$button:ident| $on_tap:block $(,)?) => {
        tap::Detector::new(Button::simple($colour, $label), |$button| $on_tap)
    };
}

impl Widget for Button {
    fn build(&self) -> Box<dyn Widget> {
        Layout::coloured(
            self.colour,
            Overflow::Hide,
            Align::Center,
            Align::Center,
            Direction::Column,
            widgets![self.label.clone()],
        )
        .into()
    }

    fn colour(&self) -> RGBA {
        self.colour
    }
}

impl Default for Button {
    fn default() -> Self {
        Self {
            label: Label::default(),
            colour: RGBA::new(0, 0, 0, 0),
            radius: Radius::new(0.0),
            borders: None,
        }
    }
}

impl Button {
    /// Creates a new button.
    pub fn new(colour: RGBA, radius: Radius, borders: [Border; 4], label: Label) -> Self {
        Self {
            colour,
            radius,
            borders: Some(borders),
            label,
        }
    }

    /// Creates the simplest button possible, without radius nor borders.
    pub fn simple(colour: RGBA, label: Label) -> Self {
        Self {
            colour,
            label,
            ..Self::default()
        }
    }

    /// Creates a button with a `radius` but no borders.
    pub fn rounded(colour: RGBA, radius: Radius, label: Label) -> Self {
        Self {
            colour,
            radius,
            label,
            ..Self::default()
        }
    }

    /// Creates a button with `borders` but no radius.
    pub fn bordered(colour: RGBA, borders: [Border; 4], label: Label) -> Self {
        Self {
            colour,
            borders: Some(borders),
            label,
            ..Self::default()
        }
    }
}
