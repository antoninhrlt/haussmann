// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{
    controllers::tap,
    graphics::{
        colours::RGBA,
        Size, draw, Point,
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
    Zone,
    view, layout, rgba, button, label, container, surface
};

use rand::Rng;
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

    let mut view = view! {
        position: [0, 0],
        size: window_size,
        layout! {
            colour: rgba!(255, 255, 255, a: 255),
            overflow: Ignore,
            wx: Align::Center,
            wy: Align::Center,
            direction: Row,
            widgets! [
                button! {
                    colour: rgba!(255, 0, 0, a: 255),
                    label: label!("red button"),
                    on_tap: |button| {
                        let mut rng = rand::thread_rng();

                        let r = rng.gen_range(0..255);
                        let g = rng.gen_range(0..255);
                        let b = rng.gen_range(0..255);
                    
                        button.colour = RGBA::new(r, g, b, 255);
                    }
                },
                layout! {
                    overflow: Hide,
                    wx: Align::Center,
                    wy: Align::Center,
                    direction: Column,
                    widgets! [
                        container! {
                            size: [150, 100],
                            widget: button![
                                colour: rgba![0, 255, 0, a: 255],
                                label: label!("green button"),
                            ]
                        },
                        container! {
                            size: [50, 50],
                            widget: button![
                                colour: rgba![0, 100, 0, a: 255],
                                label: label!("green2 button"),
                            ]
                        },
                        container! {
                            size: [50, 50],
                            widget: button![
                                colour: rgba![0, 50, 0, a: 255],
                                label: label!("green3 button"),
                            ]
                        },
                        button! {
                            colour: rgba!(0, 0, 255, a: 255),
                            label: label!("blue button"),
                            on_tap: |button| {
                                if button.colour.b <= 10 {
                                    button.colour.b = 255;
                                } else {
                                    button.colour.b -= 10;
                                }
                            }
                        }   
                    ]
                },
                surface! {
                    colour: rgba!(255, 0, 255, a: 255)
                }
            ]
        }
    };

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
                    let colour = surface.colour();
        
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

                    view.controllers::<tap::Detector<Button>>(&drawables, |controller| {
                        if tap::is_tapped(tap, controller.zone) {
                            controller.on_tap();
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
