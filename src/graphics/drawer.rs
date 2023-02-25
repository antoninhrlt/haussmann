// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{
    graphics::{Aligner, Point, Shape, Size},
    widgets::{Container, Image, Label, Layout},
    Direction, Widget, controllers::tap,
};

use super::calculate_size;

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

    /// Returns the layout governing the align rules for widgets in the drawable
    /// surface implementing this trait.
    fn layout(&self) -> Layout;

    /// Returns the size of all the containers grouped together.
    fn size_all_containers(containers: &Vec<&Container>) -> Size {
        // No container
        if containers.len() == 0 {
            return [0, 0];
        }

        let mut total_size = [0, 0];

        // Browses the sizes rather than the whole containers.
        let sizes = containers.iter().map(|container| container.size);

        for container_size in sizes {
            total_size[0] += container_size[0];
            total_size[1] += container_size[1];
        }

        total_size
    }

    /// Returns the size for each widget which is not a container. All the 
    /// non-container widgets have the same size.
    /// 
    /// The size parameter is the size of the surface containing the widgets.
    fn size_each_not_container(
        widgets: &Vec<&Box<dyn Widget>>, 
        size: &Size, 
        direction: &Direction, 
        size_all_containers: &Size
    ) -> Size {
        // Avoid divide by zero.
        assert!(widgets.len() != 0);

        match direction {
            Direction::Column => [
                // Shares the width with the other widgets.
                if size_all_containers[0] > size[0] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (size[0] - size_all_containers[0]) / widgets.len() 
                },
                // Takes the whole height.
                size[1]
            ],
            Direction::Row => [
                // Takes the whole width.
                size[0], 
                // Shares the height with the other widgets.
                if size_all_containers[1] > size[1] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (size[1] - size_all_containers[1]) / widgets.len()
                }
            ],
        }
    }

    /// The size and the position parameters are the size and the position of 
    /// the surface containing the widgets.
    /// 
    /// Does not build the layout's shape.
    fn build_shapes(layout: &Layout, size: Size) -> Vec<Shape> {
        // There is nothing to shape.
        if layout.widgets.len() == 0 {
            return vec![]
        }

        // All the widgets which are not containers.
        let not_containers = layout.not_widget::<Container>();
        
        // All the container widgets.
        let containers = layout.widgets::<Container>();

        // There are only containers, no other widgets.
        if not_containers.len() == 0 {
            // Return the shape of these containers.
            return containers
                .iter()
                .map(|container| container.shape(None))
                .collect::<Vec<Shape>>();
        }

        let size_all_containers: Size = Self::size_all_containers(&containers);
        
        let size_each_not_container: Size = Self::size_each_not_container(
            &not_containers, 
            &size, 
            &layout.direction, 
            &size_all_containers
        );

        // Builds the shape for all the widgets.
        // While calling `Widget::shape`, a `None` value is given for the 
        // containers because they have their own size.
        layout.widgets
            .iter()
            .map(|widget| match widget.as_any().downcast_ref::<Container>() {
                Some(container) => container.shape(None),
                None => widget.shape(Some(size_each_not_container)),
            })
            .collect()
    }

    /// Builds the shape for the layout being the layout of a drawable surface 
    /// with a certain size and position.
    fn build_layout_shape(layout: &Layout, position: Point, size: Size) -> Shape {
        // Creates a layout of the drawable surface's size.
        let mut layout_shape = layout.shape(Some(size));
        // Place the layout's shape at `position`.
        layout_shape.move_by(position);
        // Returns the shape.
        layout_shape
    }

    /// Draws the widgets of a layout. Recursive function when encounters a 
    /// sub-layout. 
    fn build_layout_shapes(&self, layout: &Layout, position: Point, size: Size) -> Vec<Shape> {
        // Builds the layout' shape.
        let layout_shape: Shape = Self::build_layout_shape(layout, position, size);
        
        // Builds the shapes of the layout's widgets.
        let mut widgets_shapes: Vec<Shape> = Self::build_shapes(layout, size);
        
        // Aligns the shapes following the layout's rules.
        let mut aligner = Aligner::new(&widgets_shapes);

        // Shapes which are going to be pushed at the end.
        let mut more_shapes: Vec<Shape> = vec![];
        
        // The manipulated shape is the shape of the current manipulated widget.
        for (i, shape) in widgets_shapes.iter_mut().enumerate() {
            // Aligns the shape in the layout.
            aligner.align_shape(layout, &layout_shape, shape);
 
            let widget_as_any = layout.widgets[i].as_any();
            
            // Recursive call to draw the shapes of a sub-layout.
            match widget_as_any.downcast_ref::<Layout>() {
                Some(sub_layout) => {
                    // Pushes the sub-layout's shape and its widgets' shapes in 
                    // `more_shapes` to be drawn after the other widgets.
                    more_shapes.extend(
                        self.build_layout_shapes(
                            sub_layout, 
                            shape.points()[0], 
                            calculate_size(&shape)
                        )
                    );
                }
                None => {}
            }
            
            // Since containers contain a widget and are not wrappers like the 
            // controllers, we must also shape their contained widget.
            match widget_as_any.downcast_ref::<Container>() {
                Some(container) => {
                    // Gets the shape of the container's widget.
                    // The container's widget has the same size as the 
                    // container.
                    let mut container_widget_shape = container
                        .widget
                        .shape(Some(container.size));

                    // The container's widget has the same position as the 
                    // container.
                    container_widget_shape.move_by(shape.points()[0]);
                    
                    // Pushes it into `more_shapes` to draw it after the other 
                    // shapes
                    more_shapes.push(container_widget_shape);
                }
                None => {}
            }
        }
 
        more_shapes.push(layout_shape);
        widgets_shapes.extend(more_shapes);

        widgets_shapes
    }

    /// Draws the widgets contained in the layout following the layout's rules,
    /// in a sized zone, at a certain position.
    fn draw(&mut self, position: Point, size: Size) {
        let layout = self.layout();
        
        let all_shapes = self.build_layout_shapes(&layout, position, size);
        
        for shape in &all_shapes {
            self.shape(shape);
        }
    }
}
