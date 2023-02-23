// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Image, Label, Layout, Container, Button}, graphics::{Shape, Size, Point, Aligner}, Widget, Direction};

/// Trait to implement on a drawable surface in order to draw shapes and widgets
/// which cannot be transformed into simple `Shape`s.
/// 
/// The functions are defined with a mutable `self`, it is to permit the user
/// modifying anything on their drawable surface alongside rendering widgets.
pub trait Drawer {
    /// Draws a shape on the drawable surface implementing this trait.
    fn shape(&mut self, shape: &Shape);
    
    /// Draws an image on the drawable surface implementing this trait.
    fn image(&mut self, position: Point, image: &Image);

    /// Draws a label on the drawable surface implementing this trait.
    fn label(&mut self, position: Point, label: &Label);

    fn layout(&self) -> &Layout;

    fn shapes_from_layout(&self, layout: &Layout, position: Point, size: Size) -> Vec<Shape> {
        let not_containers = layout.not_widget::<Container>();
        let containers = layout.widgets::<Container>();

        // How many not-container widgets.
        let n_widgets = not_containers.len();

        // Total of widths and heights of the widgets.
        let (mut widget_widths, mut widget_heights) = (0, 0);

        // Adds the sizes of the layouts to the total of widths and heights of 
        // the widgets, since containers have defined size.
        for container in &containers {
            widget_widths += container.size[0];
            widget_heights += container.size[1];
        }

        // The size of each widget which is not a container.
        let size_of_not_container: Size = if n_widgets == 0 {
            [0, 0]
        } else {
            match layout.direction {
                Direction::Column => [
                    (size[0] - widget_widths) / n_widgets,
                    size[1]
                ],
                Direction::Row => [
                    size[0],
                    (size[1] - widget_heights) / n_widgets, 
                ],
            }
        };

        // Gets a vector of the widgets' shape.
        let mut shapes: Vec<Shape> = vec![];

        for widget in &layout.widgets {
            // Builds the widget's shape.
            let shape = match widget.as_any().downcast_ref::<Container>() {
                Some(container) => container.shape(None),
                None => widget.shape(Some(size_of_not_container)),
            };

            shapes.push(shape);
        }
        
        let mut layout_shape = layout.shape(Some(size));
        layout_shape.move_by(position);

        // Aligns the shapes following the layout's rules.
        let mut aligner = Aligner::new(&shapes);

        let mut more_shapes = vec![];

        for (i, shape) in shapes.iter_mut().enumerate() {
            aligner.align_shape(layout, &layout_shape, shape);
            
            let widget_as_any = layout.widgets[i].as_any();

            match widget_as_any.downcast_ref::<Layout>() {
                Some(sub_layout) => {
                    // It's the shape of a layout.
                    let sub_layout_position: Point = shape.points()[0];

                    more_shapes.extend(
                        self.shapes_from_layout(
                            sub_layout,
                            sub_layout_position, 
                            size_of_not_container
                        )
                    );
                }
                None => {}
            }

            match widget_as_any.downcast_ref::<Container>() {
                Some(container) => {
                    let mut container_widget_shape = container.widget.shape(Some(container.size));
                    container_widget_shape.move_by(shape.points()[0]);
                    more_shapes.push(container_widget_shape);
                }
                None => {}
            }
        }

        shapes.extend(more_shapes);

        // Draws each shape.
        shapes.push(layout_shape);

        shapes
    }

    /// Draws the widgets contained in the layout (retrieved with 
    /// `self.layout()`) following the layout's rules, in a zone of `size` at 
    /// `position`.
    fn draw(&mut self, position: Point, size: Size) {
        let layout = self.layout();
        let shapes = self.shapes_from_layout(layout, position, size);
                
        for shape in shapes {
            self.shape(&shape);
        }
    }
}
