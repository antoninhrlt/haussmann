//! Any usage of haussmann that might be unusual and that might go into panics

use haussmann::{
    widgets::{Layout, View, Surface, Widget}, 
    widgets, 
    Align, 
    Overflow, 
    Direction,
    Zone
};

#[test]
fn no_widget() {
    let view = View::new(
        Zone {
            position: [0, 0],
            size: [1, 1]
        },
        Layout::normal(
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
        Layout::normal(
            Overflow::Ignore,
            Align::Center,
            Align::Center,
            Direction::Column,
            widgets![
                Surface::normal()
            ],
        ),
    );

    view.build();
}
