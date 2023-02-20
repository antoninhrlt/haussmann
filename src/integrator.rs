// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Button, buttons::{TextButton, ImageButton}, Image, Label, Layout}, graphics::Shape, Widget};

/// Trait to implement on a drawable surface to draw the widgets.
/// 
/// Some functions are not provided, it is because it's about widgets which 
/// cannot be rendered the normal way.
/// 
/// The functions are defined with a mutable `self`, it is to permit the user
/// modifying anything on their drawable surface alongside rendering widgets.
pub trait Integrator {
    /// Renders a shape.
    fn shape(&mut self, shape: &Shape);

    /// Renders a button.
    fn button(&mut self, button: &Button) {
        self.shape(&button.shapes()[0]);
    }
    
    /// Renders a button and renders its text over it.
    fn text_button(&mut self, text_button: &TextButton) {
        self.button(&Button::from(text_button));
        self.label(&text_button.label);
    }

    /// Renders a button and renders its image over it.
    fn image_button(&mut self, image_button: &ImageButton) {
        self.button(&Button::from(image_button));
        self.image(&image_button.image);
    }
    
    /// Renders an image.
    fn image(&mut self, image: &Image);

    /// Renders a text.
    fn label(&mut self, label: &Label);
    
    /// Renders a layout and its widgets, even the images and labels.
    /// 
    /// Of course, images and labels are rendered thanks to `image` and `label` 
    /// functions.
    fn layout(&mut self, layout: &Layout) {
        // Draws all layout's widgets having shapes.
        for shape in &layout.shapes() {
            self.shape(shape);
        }

        // Renders all the images
        for image in layout.widgets::<Image>() {
            self.image(image);
        }

        // Renders all the labels
        for label in layout.widgets::<Label>() {
            self.label(label);
        }
    }
}
