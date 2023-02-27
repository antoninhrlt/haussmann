// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{
    controllers::tap,
    graphics::{
        colours::{RGBA, self},
        Size, ShapesBuilder, Point,
    },
    widgets::{
        Button, 
        Container, 
        Label, 
        Layout,
    },
    widgets, 
    Align,
    Direction, 
    Overflow, 
    Widget,
};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
};

use std::time::Duration;

#[test]
fn with_sdl2() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // The current size of the window.
    // Updated in the events loop.
    let mut window_size: Size = [400, 600];

    let window = video_subsystem
        .window(
            "Haussmann on SDL2",
            window_size[0] as u32,
            window_size[1] as u32,
        )
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut layout = Layout::coloured(
        widgets![
            tap::Detector::new(
                Button::simple(
                    Label::simple("Button 1"), 
                    RGBA::new(255, 0, 0, 255),
                ),
                move |button| {
                    button.colour = RGBA::new(0, 255, 255, 255);
                    println!("button1 was tapped!");
                }
            ),
            Layout::simple(
                widgets![
                    Container::simple(
                        [200, 100],
                        Button::simple(
                            Label::simple("Button 3"), 
                            RGBA::new(0, 0, 255, 255),
                        )
                    ),
                    Button::simple(
                        Label::simple("Button 4"), 
                        RGBA::new(255, 255, 0, 255),
                    )
                ],
                Overflow::Hide,
                Align::Center,
                Align::Center,
                Direction::Column,
            ),
            Button::simple(
                Label::simple("Button 2"), 
                RGBA::new(0, 255, 0, 255),
            )
        ],
        RGBA::new(255, 255, 255, 255),
        Overflow::Ignore,
        Align::Center,
        Align::Center,
        Direction::Row,
    );

    let mut i = 0;

    // Where to draw the widgets.
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop {
        canvas.clear();        
        
        // Creates shapes from the widgets.
        let shapes = ShapesBuilder::new(&layout)
            .build_shapes([0, 0], window_size);
        
        // Draws the shapes.
        for shape in &shapes {
            let size = shape.size();
            
            let colour = shape
                .fill_colour()
                .unwrap_or(colours::TRANSPARENT);

            canvas.set_draw_color(Color::RGBA(
                colour.r as u8,
                colour.g as u8,
                colour.b as u8,
                colour.a as u8,
            ));

            match shape.points().len() {
                4 => {
                    canvas.fill_rect(Rect::new(
                        shape.position()[0] as i32,
                        shape.position()[1] as i32,
                        size[0] as u32,
                        size[1] as u32,
                    ))
                    .unwrap();
                }
                _ => todo!(),
            }
        }

        // Events handling
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        window_size = [width as usize, height as usize];
                    }
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn != MouseButton::Left {
                        return;
                    }

                    
                    let (x, y) = (x as isize, y as isize);

                    let s: Size = shapes[1].size();
                    let p: Point = shapes[1].position();

                    let tap_detector = layout.widgets[0]
                        .as_any_mut()
                        .downcast_mut::<tap::Detector<Button>>()
                        .unwrap();

                    if 
                        x >= p[0] && x <= p[0] + s[0] as isize 
                        && y >= p[1] && y <= p[1] + s[1] as isize 
                    {
                        tap_detector.widget.colour = RGBA::new(100, i, 100, 255);
                        //tap_detector.on_tap();
                        i += 50;
                    }
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
