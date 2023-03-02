// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use crate::{widgets::{Layout, Container}, Direction};

use super::Size;

/// Generates a vector of [`Size`] which are the sizes of every widget contained
/// in a [`Layout`].
/// 
/// Does not calculate the sizes of the sub-widgets (widgets of the layout's 
/// widgets) !
pub struct Sizer<'a> {
    layout: &'a Layout,
}

impl<'a> Sizer<'a> {
    /// Creates a new sizer for a layout.
    pub fn new(layout: &'a Layout) -> Self {
        Self {
            layout,
        }
    }

    /// Gets the size of each widget in the correct order, in a sized zone.
    /// 
    /// This sized zone is important because the widgets will together take the 
    /// whole place.
    pub fn size_in(&self, zone: Size) -> Vec<Size> {
        // No widget in the layout, no size to calculate.
        if self.layout.widgets.is_empty() {
            return vec![];
        }

        let containers = self.layout.widgets::<Container>();
        let not_containers = self.layout.not_widget::<Container>();

        // The size of all the containers grouped together.
        let mut size_containers: Size = [0, 0];
        // Calculates this size.
        for container in &containers {
            size_containers[0] += container.size[0];
            size_containers[1] += container.size[1];
        }
        
        // The layout only contains containers.
        if not_containers.len() == 0 {
            // Returns the sizes of the containers.
            return containers.iter().map(|c| c.size).collect();
        }

        // Checked by the previous condition.
        assert_ne!(not_containers.len(), 0);

        // Each widget which is not a container has the same size.
        let size_not_container: Size = match self.layout.direction {
            Direction::Column => [
                // Shares the width with the other widgets.
                if size_containers[0] > zone[0] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (zone[0] - size_containers[0]) / not_containers.len()
                },
                // Takes the whole height.
                zone[1]
            ],
            Direction::Row => [
                // Takes the whole width.
                zone[0], 
                // Shares the height with the other widgets.
                if size_containers[1] > zone[1] {
                    // Avoids 'subtract with overflow'
                    0
                } else {
                    (zone[1] - size_containers[1]) / not_containers.len()
                }
            ],
        };
        
        // Creates the sizes in the correct order.
        // The first size is the layout's size, which has the size of the zone.
        let mut sizes: Vec<Size> = vec![zone];

        for widget in &self.layout.widgets {
            // Containers have their own sizes, other widgets all have the same 
            // size.
            match widget.as_any().downcast_ref::<Container>() {
                Some(container) => {
                    sizes.push(container.size);
                    continue;
                },
                None => sizes.push(size_not_container)
            };
        }

        sizes
    }
}
