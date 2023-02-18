// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Button, buttons::{TextButton, ImageButton}, Image, Label, Layout}, graphics::{Shape, self}, Widget};

/// Trait to implement on a drawable surface to draw the widgets.
/// 
/// Some functions are not provided, it's because it's about widgets which 
/// cannot be renderer the normal way. They must be implemented.
pub trait Integrator {
    fn shape(&mut self, shape: &Shape);

    fn button(&mut self, button: &Button, in_layout: Option<&Layout>) {
        let button = &mut button.shapes()[0];
        
        // Change position if contained in a layout.
        match in_layout {
            Some(layout) => {
                if !layout.is_fixed() {
                    return;
                }

                let position = graphics::place_in_fixed_layout(layout, button);
                button.move_at([position.0, position.1]);
            }
            None => {}
        }

        self.shape(&button);
    }
    
    fn text_button(&mut self, text_button: &TextButton, in_layout: Option<&Layout>) {
        self.button(&Button::from(text_button), in_layout);
        self.label(&text_button.label, in_layout);
    }

    fn image_button(&mut self, image_button: &ImageButton, in_layout: Option<&Layout>) {
        self.button(&Button::from(image_button), in_layout);
        self.image(&image_button.image, in_layout);
    }
    
    fn image(&mut self, image: &Image, in_layout: Option<&Layout>);
    fn label(&mut self, label: &Label, in_layout: Option<&Layout>);
    
    /// Does not draw the widgets contained in the layout !
    fn layout(&mut self, layout: &Layout) {
        self.shape(&layout.shapes()[0]);
    }
}
