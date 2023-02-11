// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

use haussmann::{widgets::Widget, graphics::{ShapeBuilder, colours::RGBA, Shape}};

fn convert(shape: Shape) -> Document {
    let mut data = Data::new();

    // Top left point
    data = data.move_to((
        shape.points()[0][0],
        shape.points()[0][1],
    ));

    for point in &shape.points()[1..] {
        data = data.line_by((point[0], point[1]));
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

    Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(path)
}

#[test]
fn shape_to_svg() {
    let colour = RGBA::new(255, 0, 0, 100);

    let triangle = ShapeBuilder::<3>::new()
        .create([[0, 50], [50, 0], [0, -50]], None)
        .fill(colour)
        .finish();

    let document = convert(triangle);    
    svg::save("target/tmp/triangle.svg", &document).unwrap();
}
