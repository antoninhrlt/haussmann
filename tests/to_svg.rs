// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use svg::node::element::Path;
use svg::node::element::path::Data;

use haussmann::{widgets::{Button, CreateWidget}, graphics::{shapes, colours::RGBA, Shape}, Radius, Widget};

#[test]
fn shape_to_svg() {
    let colour = RGBA::new(255, 0, 0, 100);

    let triangle = shapes::Builder::<3>::new()
        .create([[0, 50], [50, 0], [0, -50]], None)
        .fill(colour)
        .finish();

    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 70, 70));
    
    document = svg(document, triangle);
    
    svg::save("target/tmp/triangle.svg", &document).unwrap();
}

#[test]
fn widget_to_svg() {
    let button = Button::rounded(
        [50, 20], 
        RGBA::new(255, 0, 0, 200), 
        Radius::new(20.0),
    );

    let mut document = svg::Document::new()
        .set("viewBox", (0, 0, 70, 70));

    for shape in button.shapes() { 
        document = svg(document, shape);
    }

    svg::save("target/tmp/button.svg", &document).unwrap();
}

/// Add a shape converted to SVG to a SVG document.
fn svg(document: svg::Document, shape: Shape) -> svg::Document {
    let mut data = Data::new();

    // Top left point
    data = data.move_to((
        shape.points()[0][0],   
        shape.points()[0][1],
    ));

    for point in &shape.points()[1..] {
        data = data.line_to((point[0], point[1]));

        // Corner radius can be added here from this value :
        let _ = shape.radius();
    }

    data = data.close();
    
    let fill = if shape.fill_colour() == None {
        "none".to_string()
    } else {
        format!("#{:x}", shape.fill_colour().unwrap().to_hex())
    };

    let path = Path::new()
        .set("fill", fill)
        .set("d", data);

    document.add(path)
}
