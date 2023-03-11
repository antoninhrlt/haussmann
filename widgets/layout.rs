// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{
    graphics::colours::RGBA,
    Align, Border, Direction, Overflow, ToAny,
};

use super::{DebugWidget, Surface, Widget};

/// Layout to contain several widgets and define alignment rules for these
/// widgets.
#[derive(Debug, Widget)]
pub struct Layout {
    /// The colour of the layout.
    pub colour: RGBA,
    /// The borders of the layout.
    pub borders: Option<[Border; 4]>,
    /// Rules about widget overflowing.
    pub overflow: Overflow,
    /// Widgets alignment on the X axis inside the layout.
    pub wx_align: Align,
    /// Widgets alignment on the Y axis inside the layout.
    pub wy_align: Align,
    /// The direction of the widgets arrangement.
    pub direction: Direction,
    /// The widgets contained in the layout.
    pub widgets: Vec<Box<dyn Widget>>,
}

/// Creates different layouts following the different available constructors in 
/// the [`Layout`] implementation.
#[macro_export]
macro_rules! layout {
    // Transparent
    (overflow: $overflow:ident, wx: $wx_align:expr, wy: $wy_align:expr, direction: $direction:ident, $widgets:expr $(,)?) => {
        Layout::simple(
            Overflow::$overflow,
            $wx_align,
            $wy_align,
            Direction::$direction,
            $widgets,
        )
    };
    
    // Coloured
    (colour: $colour:expr, overflow: $overflow:ident, wx: $wx_align:expr, wy: $wy_align:expr, direction: $direction:ident, $widgets:expr $(,)?) => {
        Layout::coloured(
            $colour,
            Overflow::$overflow,
            $wx_align,
            $wy_align,
            Direction::$direction,
            $widgets,
        )
    };

    // Bordered
    (borders: $borders:expr, overflow: $overflow:ident, wx: $wx_align:expr, wy: $wy_align:expr, direction: $direction:ident, $widgets:expr $(,)?) => {
        Layout::bordered(
            $colour,
            Overflow::$overflow,
            $wx_align,
            $wy_align,
            Direction::$direction,
            $widgets,
        )
    };

    // Coloured + bordered
    (colour: $colour:expr, borders: $borders:expr, overflow: $overflow:ident, wx: $wx_align:expr, wy: $wy_align:expr, direction: $direction:ident, $widgets:expr $(,)?) => {
        Layout::new(
            $colour,
            $borders,
            Overflow::$overflow,
            $wx_align,
            $wy_align,
            Direction::$direction,
            $widgets,
        )
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
            overflow: Overflow::Ignore,
            wx_align: Align::Center,
            wy_align: Align::Center,
            direction: Direction::Column,
            widgets: vec![],
        }
    }
}

impl Widget for Layout {
    fn build(&self) -> Box<dyn Widget> {
        Surface::new(self.colour, self.borders).into()
    }

    fn colour(&self) -> RGBA {
        self.colour
    }
}

impl Layout {
    /// Creates a new layout.
    pub fn new(
        colour: RGBA,
        borders: [Border; 4],
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            colour,
            borders: Some(borders),
            overflow,
            wx_align,
            wy_align,
            direction,
            widgets,
        }
    }

    /// Creates the simplest layout possible.
    pub fn simple(
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            colour: RGBA::default(),
            borders: None,
            overflow,
            wx_align,
            wy_align,
            direction,
            widgets,
        }
    }

    /// Creates a new layout with a colour.
    pub fn coloured(
        colour: RGBA,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            colour,
            borders: None,
            overflow,
            wx_align,
            wy_align,
            direction,
            widgets,
        }
    }

    /// Creates a layout with borders.
    pub fn bordered(
        borders: [Border; 4],
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            colour: RGBA::default(),
            borders: Some(borders),
            overflow,
            wx_align,
            wy_align,
            direction,
            widgets,
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

    /// Same as [`Self::widgets`] but returned widgets are 
    /// mutable references
    pub fn widgets_mut<T: 'static>(&mut self) -> Vec<&mut T> {
        let mut widgets = vec![];

        for boxed in &mut self.widgets {
            match boxed.as_any_mut().downcast_mut::<T>() {
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
