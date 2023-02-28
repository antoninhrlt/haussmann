// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::graphics::draw::Drawable;
use crate::graphics::{Sizer, Aligner, Point, Size};
use crate::Widget;

use super::{Layout, Label, Image, Surface};

/// Wraps a [`Layout`] and permit to build [`Drawable`]s from the widgets in.
/// 
/// Has a defined position and size in a drawable zone such as a window, a 
/// canvas...
pub struct View {
    position: Point,
    size: Size,
    layout: Layout,
}

impl View {
    /// Creates a new view at a certain point with a specified size.
    pub fn new(position: Point, size: Size, layout: Layout) -> Self {
        Self {
            position,
            size,
            layout,
        }
    }

    /// Creates shapes for the widgets in the layout.
    pub fn build(&self) -> Vec<Drawable> {
        Self::build_layout(&self.layout, self.position, self.size)
    }

    /// Updates the position and the size of the view, and builds again.
    pub fn rebuild(&mut self, position: Point, size: Size) -> Vec<Drawable> {
        self.position = position;
        self.size = size;

        self.build()
    }

    fn build_layout(layout: &Layout, position: Point, size: Size) -> Vec<Drawable> {
        // Initialises the drawables with the layout's drawable.
        let mut drawables = vec![];
        drawables.push(Self::built_to_drawable(layout.build(), position, size));

        // The size of every widget.
        let sizes: Vec<Size> = Sizer::new(layout)
            .size_in(size);

        // The position of every widget.
        let positions: Vec<Point> = Aligner::new(layout, sizes.clone())
            .align_at(position);

        assert_eq!(sizes.len(), positions.len());

        for (i, widget) in layout.widgets.iter().enumerate() {
            // The widget is actually a layout.
            if let Some(layout) = widget.as_any().downcast_ref::<Layout>() {
                // Builds the layout and its widgets.
                drawables.extend(
                    Self::build_layout(layout, positions[i + 1], sizes[i + 1])
                );
            
                continue;
            }

            // Builds the widget.
            drawables.extend(
                Self::build_widget(widget, positions[i + 1], sizes[i + 1])
            );
        }
        
        drawables
    }

    fn build_widget(widget: &Box<dyn Widget>, position: Point, size: Size) -> Vec<Drawable> {
        let mut drawables: Vec<Drawable> = vec![];

        // Builds the widget.
        let built = widget.build();

        // The built widget is a layout.
        if let Some(layout) = built.as_any().downcast_ref::<Layout>() {
            // Builds the layout.
            drawables.extend(
                Self::build_layout(layout, position, size)
            );
        } else {
            // Transforms the built widget into a drawable.
            drawables.push(Self::built_to_drawable(built, position, size));
        }

        drawables
    }

    /// The `built` parameter is the value returned by [`Widget::build()`].
    /// 
    /// ## Notes
    /// - [`Image::build()`] returns itself.
    /// - [`Label::build()`] returns itself. 
    /// - [`Surface::build()`] returns itself.
    /// - [`Container::build()`] returns its widget, built.
    /// - [`Layout::build()`] returns a [`Surface`].
    fn built_to_drawable(built: Box<dyn Widget>, position: Point, size: Size) -> Drawable {
        if let Some(image) = built.as_any().downcast_ref::<Image>() {
            return Drawable::Image(image.clone(), position, size);
        }

        if let Some(label) = built.as_any().downcast_ref::<Label>() {
            return Drawable::Label(label.clone(), position, size);
        }

        if let Some(surface) = built.as_any().downcast_ref::<Surface>() {
            return Drawable::Surface(surface.clone(), position, size);
        }
        
        Drawable::Unknown(built, position, size)
    }
}
