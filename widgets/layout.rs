// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{ Overflow, Align, graphics::{Size, Point}, ToAny };
use super::Widget;

/// Layout to contain several widgets.
/// 
/// It can be fixed at a certain point, in this case `x_align`, `y_align` are 
/// `None` fields but `position` is a `Some(x)`.
#[derive(Debug)]
pub struct Layout {
    /// Only used for fixed layouts.`None` for a normal layout.
    position: Option<Point>,
    /// The size of the layout is the a zone containing all the widgets, indeed
    /// no widget can be out of this zone.
    pub size: Size,
    /// The widgets contained in the layout.
    pub widgets: Vec<Box<dyn Widget>>,
    /// Rules about widget overflowing.
    pub overflow: Overflow,
    /// Layout alignment on the X axis.
    /// 
    /// `None` for fixed layouts.
    pub x_align: Option<Align>,
    /// Layout alignment on the Y axis.
    /// 
    /// `None` for fixed layouts.
    pub y_align: Option<Align>, 
    /// Widgets alignment on the X axis inside the layout.
    pub wx_align: Align,
    /// Widgets alignment on the Y axis inside the layout.
    pub wy_align: Align,
}

impl Layout {
    /// Creates a normal layout, without position defined. It is not "fixed".
    pub fn new(widgets: Vec<Box<dyn Widget>>, overflow: Overflow, x_align: Align, y_align: Align, wx_align: Align, wy_align: Align) -> Self {
        Self {
            position: None,
            size: [0, 0],
            widgets,
            overflow,
            x_align: Some(x_align),
            y_align: Some(y_align),
            wx_align,
            wy_align,
        }
    }

    /// Creates a layout fixed at a specific `position` point.
    pub fn fixed(position: Point, size: Size, widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align) -> Self {
        Self {
            position: Some(position),
            size,
            widgets,
            overflow,
            x_align: None,
            y_align: None,
            wx_align,
            wy_align,
        }
    }

    
    /// Returns all the widgets of type `T` from the `widgets` contained in the 
    /// layout.
    /// 
    /// ## Example
    /// ```rust
    /// use haussmann::widgets;
    /// use haussmann::{ Overflow, Align };
    /// use haussmann::widgets::{ Widget, SurfacedWidget, Button, FixedLayout };
    /// use haussmann::graphics::colours::RGBA;
    /// 
    /// let button1 = Button::new([30, 10], RGBA::new(0, 255, 255, 255));
    /// let button2 = Button::new([30, 10], RGBA::new(255, 0, 255, 255));
    ///
    /// let layout = FixedLayout::new(
    ///     [0, 0], [100, 100], 
    ///     widgets![button1, button2],
    ///     Overflow::Hide,
    ///     Align::Center, Align::Center
    /// );
    /// let buttons = layout.widgets::<Button>();
    /// ```
    pub fn widgets<T: 'static>(&self) -> Vec<&T> {
        let mut widgets = vec![];

        for boxed in &self.widgets {
            let any: &dyn std::any::Any = boxed.as_any();

            match any.downcast_ref::<T>() {
                Some(widget) => widgets.push(widget.clone()),
                None => {} // not a widget of type `T`.
            }
        }
        
        widgets
    } 

    /// Returns whether the layout is "fixed" or not. 
    pub fn is_fixed(&self) -> bool {
        self.position != None
    }

    /// Returns the `position` when it is not `None`. Otherwise, panics.
    pub fn position(&self) -> Point {
        self.position.expect("unable to get the position of a not-fixed layout")
    }
}
