// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::graphics::{self, Shape};
use haussmann::graphics::colours::RGBA;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

use haussmann::{widgets, Align, Widget, Overflow};
use haussmann::widgets::{Button, Label, Layout};

#[test]
fn with_sdl2() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Haussmann on SDL2", 400, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas()
        .present_vsync()
        .build()
        .unwrap();

    let label = Label::new("Hello!");

    let button = Button::simple(
        [100, 50], 
        RGBA::new(255, 0, 0, 255),
    );

    let layout = Layout::fixed(
        [50, 50],
        [300, 500],
        widgets![button, label], 
        Overflow::Hide, 
        Align::Center, 
        Align::Center
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        
        canvas.set_draw_color(Color::RGBA(0, 0, 0, 100));

        canvas.fill_rect(
            Rect::new(
                layout.position()[0] as i32, 
                layout.position()[1] as i32, 
                layout.size[0] as u32, 
                layout.size[1] as u32
            )
        ).unwrap();

        let button: &Button = layout.widgets::<Button>()[0];
        let button: Shape = button.shapes()[0].clone();

        let button_colour = button.fill_colour().unwrap();
        canvas.set_draw_color(
            Color::RGBA(
                button_colour.r as u8, 
                button_colour.g as u8, 
                button_colour.b as u8, 
                button_colour.a as u8
            )
        );
        
        let (button_weight, button_height) = graphics::calculate_size(&button);
        let (button_x, button_y) = graphics::place_in_fixed_layout(&layout, &button);

        canvas.fill_rect(
            Rect::new(
                button_x as i32, 
                button_y as i32, 
                button_weight as u32, 
                button_height as u32
            )
        ).unwrap();

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
} 
