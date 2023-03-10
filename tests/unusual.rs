//! Any usage of haussmann that might be unusual and that might go into panics

use haussmann::{view, layout, Align, widgets::{Layout, View, Surface, Widget}, widgets, Overflow, Direction, rgba, graphics::colours::RGBA, Zone};

#[test]
fn no_widget() {
    let view = View::new(
        Zone {
            position: [0, 0],
            size: [1, 1]
        },
        Layout::simple(
            Overflow::Ignore,
            Align::Center,
            Align::Center,
            Direction::Column,
            widgets![],
        ),
    );

    view.build();
}

#[test]
fn void_view() {
    let view = View::new(
        Zone {
            position: [0, 0],
            size: [0, 0]
        },
        Layout::simple(
            Overflow::Ignore,
            Align::Center,
            Align::Center,
            Direction::Column,
            widgets![
                Surface::coloured(rgba!(255, 255, 255, a: 255))
            ],
        ),
    );

    view.build();
}
