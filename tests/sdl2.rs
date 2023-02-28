// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{
    controllers::tap,
    graphics::{
        colours::{RGBA, self},
        Size, Point, draw::Drawable,
    },
    widgets::{
        Button, 
        Container, 
        Label, 
        Layout, View, Surface,
    },
    widgets, 
    Align,
    Direction, 
    Overflow, 
    Widget,
};

use rand::Rng;
use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect, surface,
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

    let mut view = View::new(
        [0, 0],
        window_size,
        Layout::coloured(
            widgets![
                tap::Detector::new(
                    Button::simple(
                        Label::simple("red button"), 
                        RGBA::new(255, 0, 0, 255),
                    ),
                    |button| {
                        let mut rng = rand::thread_rng();

                        let r = rng.gen_range(0..255);
                        let g = rng.gen_range(0..255);
                        let b = rng.gen_range(0..255);
                    
                        button.colour = RGBA::new(r, g, b, 255);
                    }
                ),
                Layout::simple(
                    widgets![
                        Container::simple(
                            [150, 100],
                            Button::simple(
                                Label::simple("green button"), 
                                RGBA::new(0, 255, 0, 255),
                            )
                        ),
                        tap::Detector::new(
                            Button::simple(
                                Label::simple("blue button"), 
                                RGBA::new(0, 0, 255, 255),
                            ),
                            |button| {
                                button.colour.b -= 10;
                            }
                        )
                    ],
                    Overflow::Hide,
                    Align::Center,
                    Align::Center,
                    Direction::Column,
                ),
                Surface::coloured(
                    RGBA::new(255, 0, 255, 255),
                )
            ],
            RGBA::new(255, 255, 255, 255),
            Overflow::Ignore,
            Align::Center,
            Align::Center,
            Direction::Row,
        )
    );

    // Where to draw the widgets.
    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap();
    
    canvas.set_blend_mode(sdl2::render::BlendMode::Blend);
    
    let mut event_pump = sdl_context.event_pump().unwrap();
    
    // Creates drawables from the widgets.
    let mut drawables = view.build();
    
    'running: loop {
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 0));
        canvas.clear();        
            
        // Draws the drawables.
        for drawable in &drawables {
            match drawable {
                Drawable::Surface(surface, position, size) => {
                    let colour = surface.colour();
        
                    canvas.set_draw_color(Color::RGBA(
                        colour.r as u8,
                        colour.g as u8,
                        colour.b as u8,
                        colour.a as u8,
                    ));
        
                    canvas.fill_rect(Rect::new(
                        position[0] as i32,
                        position[1] as i32,
                        size[0] as u32,
                        size[1] as u32,
                    ))
                    .unwrap();
                }
                Drawable::Image(image, position, size) => {
                    //println!("draws image {:?}", image);
                }
                Drawable::Label(label, position, size) => {
                    //println!("draws label {:?}", label);
                }
                Drawable::Unknown(widget, position, size) => {
                    println!("unknown widget to draw : {:?}", widget);
                }
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

                        // Rebuilds the view to fit the new window's size.
                        drawables = view.rebuild([0, 0], window_size);
                    }
                    _ => {}
                },
                Event::MouseButtonDown {
                    mouse_btn, x, y, ..
                } => {
                    if mouse_btn != MouseButton::Left {
                        return;
                    }

                    // let tap: Point = [x as isize, y as isize];

                    // for tap_detector in view.controllers::<tap::Detector<Button>>() {
                    //     if tap::is_tapped(tap, tap_detector.zone.0, tap_detector.zone.1) {
                    //         tap_detector.on_tap();
                    //     }
                    // }

                    // drawables = view.rebuild([0, 0], window_size);
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
