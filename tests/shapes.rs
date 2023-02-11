// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::graphics::{ ShapeBuilder, colours::RGBA, };

#[test]
fn shape_builder() {
    let a = [10, 10];
    let b = [110, 10];
    let c = [110, 110];

    let colour = RGBA::new(255, 0, 0, 255);

    let triangle = ShapeBuilder::<3>::new()
        .create([a, b, c], None)
        .fill(colour)
        .finish();

    assert_eq!(*triangle.points(), vec![a, b, c]);
    assert_eq!(triangle.borders(), None);
    assert_eq!(triangle.is_filled(), true);
    assert_eq!(triangle.fill_colour(), Some(colour));
}
