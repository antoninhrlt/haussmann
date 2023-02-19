// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{ Overflow, Align, graphics::{Size, Point, shapes::{self, Shape}, colours::RGBA, self, Aligner}, Border, ToAny, Direction, direction };
use super::{ Widget, DebugWidget };

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
    size: Size,
    /// The colour of the layout.
    pub colour: RGBA,
    /// The borders of the layout.
    pub borders: Option<[Border; 4]>,
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
    /// The direction of the widgets arrangement.
    pub direction: Direction,
}

crate::dynamic_widget!(Layout);

impl Widget for Layout {
    /// Returns the layout's shape plus the shapes of its widgets.
    fn shapes(&self) -> Vec<Shape> {
        let mut shapes = vec![];
        
        let mut layout_shape = shapes::Builder::new()
            .rectangle(self.size, self.borders)
            .fill(self.colour)
            .finish();

        let is_fixed: bool = self.is_fixed();

        // Moves the layout and push its shape. 
        if is_fixed {
            layout_shape.move_by(self.position.unwrap());
            shapes.push(layout_shape);
        }

        let mut aligner = Aligner::new(&self);

        // Pushes all the shapes of the widgets contained in the layout.
        for widget in &self.widgets {
            for shape in &widget.shapes() {
                // Creates a copy to be able to move the shape.
                let mut copy = shape.clone();

                if is_fixed {
                    copy.move_by(aligner.align(shape));
                }

                shapes.push(copy);
            }
        }

        shapes
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl Layout {
    /// Creates a normal layout, without position defined. It is not "fixed".
    pub fn new(widgets: Vec<Box<dyn Widget>>, overflow: Overflow, x_align: Align, y_align: Align, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: None,
            size: [0, 0],
            colour: RGBA::default(),
            borders: None,
            widgets,
            overflow,
            x_align: Some(x_align),
            y_align: Some(y_align),
            wx_align,
            wy_align,
            direction
        }
    }
    
    /// Creates a normal layout with borders, without position defined. It is 
    /// not "fixed".
    pub fn bordered(size: Size, borders: [Border; 4], widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: None,
            size,
            colour: RGBA::default(),
            borders: Some(borders),
            widgets,
            overflow,
            x_align: None,
            y_align: None,
            wx_align,
            wy_align,
            direction
        }
    }

    /// Creates a layout fixed at a specific `position` point.
    pub fn fixed(position: Point, size: Size, widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: Some(position),
            size,
            colour: RGBA::default(),
            borders: None,
            widgets,
            overflow,
            x_align: None,
            y_align: None,
            wx_align,
            wy_align,
            direction
        }
    }

    /// Creates a layout with borders, fixed at a specific `position` point.
    pub fn fixed_bordered(position: Point, size: Size, borders: [Border; 4], widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: Some(position),
            size,
            colour: RGBA::default(),
            borders: Some(borders),
            widgets,
            overflow,
            x_align: None,
            y_align: None,
            wx_align,
            wy_align,
            direction
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

    /// Returns whether the layout is "fixed" or not. 
    pub fn is_fixed(&self) -> bool {
        self.position != None
    }

    /// Returns the `position` when it is not `None`. Otherwise, panics.
    pub fn position(&self) -> Point {
        self.position.expect("unable to get the position of a not-fixed layout")
    }
}
