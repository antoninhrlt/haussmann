// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{
    widgets::{ Widget, Layout, View, Label }, 
    graphics::{ Point, Size },
    widgets,
    Align,
    Direction,
    Overflow,
    Zone,
};

#[test]
fn for_web() {
    let view = View::new(
        // Works differently as a GUI tool. Here, the zone does not matter nor 
        // the calculations that might be done to place the widgets.
        Zone {
            position: Point::from([0, 0]),
            size: Size::from([0, 0]),
        },
        Layout::normal(
            Overflow::Hide, 
            Align::Center, 
            Align::Center, 
            Direction::Column, 
            widgets![
                Label::normal("Haussmann for the web!"),
            ],
        ),
    );

    let drawables = view.build();
}
