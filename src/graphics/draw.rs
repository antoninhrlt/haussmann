// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

//! Everything related to drawing of widgets.

use crate::{widgets::{Label, Image, Surface, Layout, View}, Widget, Zone, graphics::{Sizer, Aligner}};

use super::{Size, Point};

/// The object to draw from a [`Drawable`].
#[derive(Debug)]
pub enum Object {
    /// Wraps an [`Image`] in order to draw it.
    Image(Image),
    /// Wraps an [`Label`] in order to draw it.
    Label(Label),
    /// Wraps an [`Surface`] in order to draw it.
    Surface(Surface),
    /// Wraps a boxed dynamic [`Widget`] in order to draw it.
    Unknown(Box<dyn Widget>),
}

impl From<Image> for Object {
    fn from(value: Image) -> Self {
        Self::Image(value)
    }
}

impl From<Label> for Object {
    fn from(value: Label) -> Self {
        Self::Label(value)
    }
}

impl From<Surface> for Object {
    fn from(value: Surface) -> Self {
        Self::Surface(value)
    }
}

impl From<Box<dyn Widget>> for Object {
    fn from(value: Box<dyn Widget>) -> Self {
        Self::Unknown(value)
    }
}

/// Something that can be drawn and which has a defined position and size 
/// calculated at generation.
#[derive(Debug)]
pub struct Drawable {
    /// The object to be drawn.
    pub object: Object,
    /// Identifier for the group this drawable belongs to. A group for a 
    /// drawable is all the drawables created by the same built widget.
    pub group_id: i32,
    /// The zone covered by the drawable.
    pub zone: Zone,
}

impl Drawable {
    /// Creates a new drawable from an object at a defined position and size.
    pub fn new<T: Into<Object>>(object: T, zone: Zone, group_id: i32) -> Self {
        Self {
            object: object.into(),
            group_id,
            zone,
        }
    }
}

pub trait DrawableAt {
    fn at(&self, i: usize) -> Option<&Drawable>;
}

impl DrawableAt for Vec<Drawable> {
    fn at(&self, i: usize) -> Option<&Drawable> {
        for drawable in self {
            if drawable.group_id as usize == i {
                return Some(drawable);
            }
        }

        None
    }
}

/// Builds drawables for a vector of widgets.
/// 
/// The widgets of the layouts found in the vector of widgets are also built.
#[derive(Debug)]
pub struct Builder {
    /// Current created drawables.
    pub drawables: Vec<Drawable>,
    /// The zone where to build the drawables.
    pub zone: Zone,
    /// The identifier of the current manipulated widget.
    /// 
    /// It is given to the drawable built by the widget.
    current_id: i32,
    /// The current manipulated zone.
    current_zone: Zone,
}

impl Builder {
    /// Creates a new drawables builder for a vector of widgets. 
    pub fn new(zone: Zone) -> Self {
        Self {
            zone,
            drawables: vec![],
            current_id: 0,
            current_zone: zone,
        }
    }

    /// Builds the widgets contained in the view's layout.
    pub fn build_view(&mut self, view: &View) {
        self.build_layout(&view.layout, false);
    }
}

impl Builder {
    /// Builds drawables for a layout. Adds the layout's drawable.
    /// 
    /// Recursive when another layout is encountered is the layout's widgets.
    fn build_layout(&mut self, layout: &Layout, from_built: bool) {
        // Builds the layout's surface.
        let layout_surface = layout.build();
        // Creates a drawable for it.
        let layout_drawable = self.create_drawable(layout_surface);
        // Pushes it to the drawables.
        self.drawables.push(layout_drawable);

        // The size of every widget.
        let sizes: Vec<Size> = Sizer::new(layout)
            .size_in(self.current_zone.size);

        // The position of every widget.
        let positions: Vec<Point> = Aligner::new(layout, sizes.clone())
            .align_at(self.current_zone.position);

        // There must be the same number of sizes than positions. 
        assert_eq!(sizes.len(), positions.len());

        // Iterates over the layout's widgets and build them has a normal 
        // widgets or layouts following their actual types.
        for (i, widget) in layout.widgets.iter().enumerate() {
            if !from_built {
                self.current_id += 1;
            }
            
            // Updates the current drawable zone.
            self.current_zone = (positions[i + 1], sizes[i + 1]).into();

            // Checks if it is a layout or a normal widget.
            if let Some(layout) = widget.as_any().downcast_ref::<Layout>() {
                // Builds the layout.
                self.build_layout(&layout, false);
            } else {
                // Builds drawables for the widget.
                self.build_widget(widget);
            }
        }
    }
    
    /// Builds drawables for a widget. Adds new drawables to the `drawables`.
    fn build_widget(&mut self, widget: &Box<dyn Widget>) {
        // Builds the widget.
        // The built widget can be a layout.
        let built = widget.build();

        // Checks for built widget to be a layout.
        if let Some(layout) = built.as_any().downcast_ref::<Layout>() {
            // Builds the layout.
            self.build_layout(layout, true);
            return;
        }
        
        // The built widget is not a layout.

        // Creates the drawable for the built widget.
        let drawable = self.create_drawable(built);
        // Adds the drawable of the built widget to the created drawables.
        self.drawables.push(drawable);
    }

    /// Creates a drawable for the returned widget by [`Widget::build()`].
    fn create_drawable(&self, built: Box<dyn Widget>) -> Drawable {
        // The built widget is an image.
        if let Some(image) = built.as_any().downcast_ref::<Image>() {
            return Drawable::new(
                image.clone(), 
                self.current_zone, 
                self.current_id
            );
        }

        // The built widget is a label.
        if let Some(label) = built.as_any().downcast_ref::<Label>() {
            return Drawable::new(
                label.clone(), 
                self.current_zone, 
                self.current_id
            );
        }

        // The built widget is a surface.
        if let Some(surface) = built.as_any().downcast_ref::<Surface>() {
            return Drawable::new(
                surface.clone(), 
                self.current_zone, 
                self.current_id
            );
        }
        
        // The built widget is not an image, a label nor a surface.
        Drawable::new(built, self.current_zone, self.current_id)
    }
}
