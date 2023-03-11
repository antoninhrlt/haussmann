// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann_dev::Widget;

use crate::{ Align, Direction, Overflow, ToAny, themes::{Theme, Style} };

use super::{DebugWidget, Surface, Widget};

/// Layout to contain several widgets and define alignment rules for these
/// widgets.
#[derive(Debug, Widget)]
pub struct Layout {
    /// The style of the layout.
    pub style: Option<Style>,
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

impl Default for Layout {
    fn default() -> Self {
        Self {
            style: None,
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
        Surface::new(self.style.clone()).into()
    }
    
    fn style(&self, theme: &Theme) -> Style {
        match &self.style {
            Some(style) => style,
            None => &theme.style
        }
        .clone()
    }
    
    fn style_mut(&mut self, theme: &Theme) -> &mut Style {
        if let None = self.style {
            self.style = Some(theme.style.clone()); 
        }

        self.style.as_mut().unwrap()
    }
}

impl Layout {
    /// Creates a new layout with an independent style from the global theme.
    pub fn styled(
        style: Style,
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            style: Some(style),
            overflow,
            wx_align,
            wy_align,
            direction,
            widgets,
        }
    }
    /// Creates a new layout without independent theme.
    pub fn normal(
        overflow: Overflow,
        wx_align: Align,
        wy_align: Align,
        direction: Direction,
        widgets: Vec<Box<dyn Widget>>,
    ) -> Self {
        Self {
            style: None,
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
