// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use std::{time::Duration, borrow::Borrow};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    mouse::MouseButton,
    pixels::Color,
    rect::Rect,
};

use rand::Rng;

use haussmann::{
    controllers::tap, 
    graphics::{Size, draw, Point, colours::{self, RGBA}}, 
    themes::{self, Style},  
    widgets::*, 
    Align,
    Border,
    Direction, 
    Overflow,
    Side,  
    Zone, 
    button, 
    rgba, 
    style,
    widgets,
};

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

    let theme = themes::default(vec![]);

    let mut view = View::new(
        Zone {
            position: [0, 0],
            size: window_size,
        },
        Layout::styled(
            style!(colour: rgba!(255, 255, 255, a: 255)),
            Overflow::Ignore,
            Align::Center,
            Align::Center,
            Direction::Row,
            widgets! [
                button!(
                    style!(
                        colour: rgba!(255, 0, 0, a: 255),
                        borders: [
                            None,
                            Some(Border::new(5, rgba!(0, 0, 0, a: 255), Side::Bottom)),
                            Some(Border::new(10, rgba!(0, 0, 0, a: 255), Side::Top)),
                            None,
                        ]
                    ),
                    Label::normal("red button"),
                    on_tap: |button, theme| {
                        let mut rng = rand::thread_rng();

                        let r = rng.gen_range(0..255);
                        let g = rng.gen_range(0..255);
                        let b = rng.gen_range(0..255);
                    
                        button.style_mut(&theme).colour = Some(rgba!(r, g, b, a: 255));
                    }
                ),
                Layout::styled(
                    style!(colour: colours::TRANSPARENT),
                    Overflow::Hide,
                    Align::Center,
                    Align::Center,
                    Direction::Column,
                    widgets! [
                        Container::new(
                            [150, 100],
                            Button::normal(
                                Label::normal("default-styled button"),
                            )
                        ),
                        Container::new(
                            [50, 50],
                            Button::styled(
                                style!(colour: rgba![0, 50, 0, a: 255]),
                                Label::normal("green3 button"),
                            )
                        ),
                        button!(
                            style!(colour: rgba!(0, 0, 255, a: 255)),
                            Label::normal("blue button"),
                            on_tap: |button, theme| {
                                let b_channel = &mut button.style_mut(&theme).colour.as_mut().unwrap().b;

                                if *b_channel <= 10 {
                                    *b_channel = 255;
                                } else {
                                    *b_channel -= 10;
                                }
                            }
                        ),
                    ]
                ),
                Surface::normal(),
            ]
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
            let zone = drawable.zone;

            match &drawable.object {
                draw::Object::Surface(surface) => {
                    let colour = surface.style(&theme).colour.unwrap();

                    canvas.set_draw_color(Color::RGBA(
                        colour.r as u8,
                        colour.g as u8,
                        colour.b as u8,
                        colour.a as u8,
                    ));
        
                    canvas.fill_rect(Rect::new(
                        zone.x() as i32,
                        zone.y() as i32,
                        zone.width() as u32,
                        zone.height() as u32,
                    ))
                    .unwrap();

                    // Draws the borders of this surface.
                    let borders = surface.style(&theme)
                        .borders
                        .unwrap_or([None, None, None, None]);
                    
                    for border in borders {
                        if border == None {
                            continue;
                        }

                        let zone = border.clone().unwrap().as_zone(&zone);
                        let colour = border.unwrap().colour;

                        canvas.set_draw_color(Color::RGBA(
                            colour.r as u8,
                            colour.g as u8,
                            colour.b as u8,
                            colour.a as u8,
                        ));

                        canvas.fill_rect(Rect::new(
                            zone.x() as i32,
                            zone.y() as i32,
                            zone.width() as u32,
                            zone.height() as u32,
                        ))
                        .unwrap();
                    }
                }
                draw::Object::Image(_) => {
                    //println!("draws image {:?}", image);
                }
                draw::Object::Label(_) => {
                    //println!("draws label {:?}", label);
                }
                draw::Object::Unknown(widget) => {
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

                    let tap: Point = [x as isize, y as isize];

                    view.controllers::<tap::Detector<Button>>(drawables, |controller| {
                        if tap::is_tapped(tap, controller.zone) {
                            controller.on_tap(&theme);
                        }
                    });

                    drawables = view.rebuild([0, 0], window_size);
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
