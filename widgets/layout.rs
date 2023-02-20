// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{ Overflow, Align, graphics::{Size, Point, shapes::{self, Shape}, colours::RGBA, Aligner}, Border, ToAny, Direction };
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
        // Creates a shape for the layout.
        let mut layout_shape = shapes::Builder::new()
            .rectangle(self.size, self.borders)
            .fill(self.colour)
            .finish();

        // Whether this layout is fixed or not.
        let is_fixed: bool = self.is_fixed();

        if is_fixed {
            // Moves the layout's shape to the layout's actual position. 
            layout_shape.move_by(self.position.unwrap());
        }

        let mut shapes = vec![];
        // Pushes the layout's shape.
        shapes.push(layout_shape);

        // Pushes the widgets' shape.
        for widget in &self.widgets {
            let widget_shapes = &widget.shapes();

            // Widget is not shaped.
            if widget_shapes.len() == 0 {
                continue;
            }

            // todo: sub-layouts
            shapes.push(widget_shapes[0].clone());
        }

        if is_fixed {
            // Align the `shapes` in the layout.
            // Directly modify the elements of the `shapes` vector.
            let mut aligner = Aligner::new(self);
            aligner.align_shapes(&mut shapes);
        }

        shapes
    }

    fn size(&self) -> Size {
        self.size
    }
}

impl Layout {
    /// Creates a normal layout, without position defined. It is not "fixed".
    pub fn new(size: Size, widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: None,
            size,
            colour: RGBA::default(),
            borders: None,
            widgets,
            overflow,
            wx_align,
            wy_align,
            direction
        }
    }

    pub fn coloured(size: Size, widgets: Vec<Box<dyn Widget>>, colour: RGBA, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: None,
            size,
            colour,
            borders: None,
            widgets,
            overflow,
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
            wx_align,
            wy_align,
            direction
        }
    }

    pub fn fixed_coloured(position: Point, size: Size, colour: RGBA, widgets: Vec<Box<dyn Widget>>, overflow: Overflow, wx_align: Align, wy_align: Align, direction: Direction) -> Self {
        Self {
            position: Some(position),
            size,
            colour,
            borders: None,
            widgets,
            overflow,
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
        self.position.expect("unable to get the position of a non-fixed layout")
    }
}
