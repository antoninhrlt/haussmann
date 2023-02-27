// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Layout, Container}, Direction, Widget};
use super::{Shape, Size, Point, aligner::Aligner};

#[derive(Debug)]
pub struct ShapesBuilder<'a> {
    layout: &'a Layout
}

impl<'a> ShapesBuilder<'a> {
    /// Creates a new shapes builder for a layout.
    pub fn new(layout: &'a Layout) -> Self {
        Self {
            layout,
        }
    }

    fn build_containers_pre_shapes(containers: &Vec<&Container>) -> Vec<Shape> {
        containers
            .iter()
            .map(|container| {
                // The position is (x=0; y=0) because it's a pre-shape. The 
                // position is going to be defined, but later.
                //
                // The size is `None`s  since a container has its own size 
                // not depending on the other widgets.
                container.shape(Some([0, 0]), None)
            })
            .collect::<Vec<Shape>>()
    }

    /// Builds shapes which have no defined position, consequently they are 
    /// pre-shapes.
    fn build_pre_shapes(&self, zone_size: Size) -> Vec<Shape> {
        // No widgets at all.
        if self.layout.widgets.len() == 0 {
            return vec![];
        }

        // They have a defined size.
        let containers = self.layout.widgets::<Container>();

        // Size of all the containers grouped together.
        let mut size_containers: Size = [0, 0];
        // Calculates this size.
        for container in &containers {
            size_containers[0] += container.size[0];
            size_containers[1] += container.size[1];
        }

        // All the widgets not being containers.
        let widgets = self.layout.not_widget::<Container>();

        // No widgets (not being containers), but there are containers.
        // There are necessarily containers since a check fot "no widgets at 
        // all" has been done at the beginning.
        if widgets.len() == 0 {
            return Self::build_containers_pre_shapes(&containers);
        }

        // Size for each widget which is not a container.
        let size_each: Size = match self.layout.direction {
            Direction::Column => [
                // Shares the width with the other widgets.
                if size_containers[0] > zone_size[0] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (zone_size[0] - size_containers[0]) / widgets.len() 
                },
                // Takes the whole height.
                zone_size[1]
            ],
            Direction::Row => [
                // Takes the whole width.
                zone_size[0], 
                // Shares the height with the other widgets.
                if size_containers[1] > zone_size[1] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (zone_size[1] - size_containers[1]) / widgets.len()
                }
            ],
        };

        // Builds pre shapes for the widgets not being containers.
        // The position of the shape is not already known but the size is.
        let mut pre_shapes: Vec<Shape> = vec![];

        // The position is (x=0; y=0) because it's a pre-shape. The position is
        // going to be defined, but later.

        for widget in &self.layout.widgets {
            // Builds the widget's shape differently according to it is a 
            // container or not.
            let widget_shape = match widget.as_any().downcast_ref::<Container>() {
                Some(_) => widget.shape(Some([0, 0]), None),
                None => widget.shape(Some([0, 0]), Some(size_each)),
            };

            pre_shapes.push(widget_shape);
        }

        // Returns these pre-shapes.
        pre_shapes
    }

    /// Builds the shapes of the widgets contained in the layout in a drawable 
    /// sized zone.
    pub fn build_shapes(&self, zone_position: Point, zone_size: Size) -> Vec<Shape> {
        // Builds the shape of the layout.
        let layout_shape = self.layout.shape(
            Some(zone_position), 
            Some(zone_size)
        );

        let mut shapes: Vec<Shape> = vec![];
        
        // Builds shapes without defined position, they are pre-shapes.
        shapes.extend(self.build_pre_shapes(zone_size));

        // The shapes being built during the loop.
        let mut new_shapes: Vec<Shape> = vec![];

        // Aligns the pre-shapes inside the layout.
        let mut aligner = Aligner::new(&shapes);

        // Aligns each shape and may build new shapes if widgets have child/ren. 
        for (i, shape) in shapes.iter_mut().enumerate() {
            // Aligns the shape in the layout.
            aligner.align_shape(&self.layout, &layout_shape, shape);

            // The widget which has created this `shape`.
            let widget = &self.layout.widgets[i];
            // Manipulates the widget without knowing its type.
            let widget = widget.as_any();

            match widget.downcast_ref::<Layout>() {
                Some(sub_layout) => {
                    // Note: the layout is already aligned, its shape is no more 
                    // a pre-shape.
                    let layout_position: Point = shape.position();

                    let layout_size: Size = shape.size();

                    // Build the shapes of this sub-layout.
                    new_shapes.extend(
                        Self::new(sub_layout)
                            .build_shapes(
                                layout_position, 
                                layout_size
                            )
                    );
                }
                None => {}
            }

            // Containers have a child widget.
            match widget.downcast_ref::<Container>() {
                Some(container) => {
                    // The container's widget has the same position as the 
                    // container.
                    //
                    // Note: the container is already aligned, its shape is no 
                    // more a pre-shape.
                    let position = shape.points()[0];

                    // Gets the shape of the container's widget.
                    // The container's widget has the same size as the 
                    // container.
                    let container_widget_shape = container
                        .widget
                        .shape(Some(position), Some(container.size));
 
                    new_shapes.push(container_widget_shape);
                }
                _ => {} // not a container
            }
        }
        
        // Adds the shapes built during the loop.
        shapes.insert(0, layout_shape);
        shapes.extend(new_shapes);
        shapes
    }
}
