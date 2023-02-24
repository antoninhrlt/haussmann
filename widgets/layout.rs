// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use super::{DebugWidget, Widget};
use crate::{
    graphics::{
        colours::RGBA,
        shapes::{self, Shape},
        Size,
    },
    Align, Border, Direction, Overflow, ToAny,
};

/// Layout to contain several widgets and define alignment rules for these
/// widgets.
#[derive(Debug)]
pub struct Layout {
    /// The colour of the layout.
    pub colour: RGBA,
    /// The borders of the layout.
    pub borders: Option<[Border; 4]>,
    /// The widgets contained in the layout.
    pub widgets: Vec<Box<dyn Widget>>,
    /// Rules about widget overflowing.
    pub overflow: Overflow,
    /// Widgets alignment on the X axis inside the layout.
    pub wx_align: Align,
    /// Widgets alignment on the Y axis inside the layout.
    pub wy_align: Align,
    /// The direction of the widgets arrangement.
    pub direction: Direction,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
            widgets: vec![],
            overflow: Overflow::Ignore,
            wx_align: Align::Center,
            wy_align: Align::Center,
            direction: Direction::Column,
        }
    }
}

crate::dynamic_widget!(Layout);

impl Widget for Layout {
    /// Returns the shape of the layout itself of `size` filled with colour
    /// `self.colour`, with borders if defined.
    fn shape(&self, size: Option<Size>) -> Shape {
        shapes::Builder::new()
            .rectangle(size.unwrap(), self.borders)
            .fill(self.colour)
            .finish()
    }
}

impl Layout {
    /// Creates a new layout.
    pub fn new(
        colour: RGBA,
        borders: [Border; 4],
        widgets: Vec<Box<dyn Widget>>,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
    ) -> Self {
        Self {
            colour,
            borders: Some(borders),
            widgets,
            overflow,
            wx_align,
            wy_align,
            direction,
        }
    }

    /// Creates the simplest layout possible.
    pub fn simple(
        widgets: Vec<Box<dyn Widget>>,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
    ) -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
            widgets,
            overflow,
            wx_align,
            wy_align,
            direction,
        }
    }

    /// Creates a new layout with a colour.
    pub fn coloured(
        widgets: Vec<Box<dyn Widget>>,
        colour: RGBA,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
    ) -> Self {
        Self {
            colour,
            borders: None,
            widgets,
            overflow,
            wx_align,
            wy_align,
            direction,
        }
    }

    /// Creates a layout with borders.
    pub fn bordered(
        borders: [Border; 4],
        widgets: Vec<Box<dyn Widget>>,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
    ) -> Self {
        Self {
            colour: RGBA::default(),
            borders: Some(borders),
            widgets,
            overflow,
            wx_align,
            wy_align,
            direction,
        }
    }

    /// Returns all the widgets of type `T` from the `widgets` contained in the
    /// layout.
    ///
    /// ## Example
    /// ```rust
    /// use haussmann::widgets;
    /// use haussmann::{ Overflow, Align };
    /// use haussmann::widgets::{ Widget, Button, Label, Layout };
    /// use haussmann::graphics::colours::RGBA;
    ///
    /// let button1 = Button::simple([30, 10], RGBA::new(0, 255, 255, 255));
    /// let button2 = Button::simple([30, 10], RGBA::new(255, 0, 255, 255));
    /// let label1 = Label::simple("label1");
    ///
    /// let layout = Layout::fixed(
    ///     [0, 0], [100, 100],
    ///     widgets![button1, label1, button2],
    ///     Overflow::Hide,
    ///     Align::Center, Align::Center
    /// );
    /// let labels = layout.widgets::<Label>();
    ///
    /// assert_eq!(labels, vec![&Label::simple("label1")]);
    /// ```
    pub fn widgets<T: 'static>(&self) -> Vec<&T> {
        let mut widgets = vec![];

        for boxed in &self.widgets {
            match boxed.as_any().downcast_ref::<T>() {
                Some(widget) => widgets.push(widget),
                None => {} // not a widget of type `T`.
            }
        }

        widgets
    }

    /// Returns all the widgets which are not of type `T`.
    pub fn not_widget<T: 'static>(&self) -> Vec<&Box<dyn Widget>> {
        let mut widgets = vec![];

        for boxed in &self.widgets {
            match boxed.as_any().downcast_ref::<T>() {
                Some(_) => {} // type `T`, ignore it.
                None => widgets.push(boxed),
            }
        }

        widgets
    }
}
