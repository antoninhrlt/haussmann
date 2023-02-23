// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{Drawer, Direction, Theme, TextTheme, FontWeight, TextAlign};
use haussmann::graphics::{Shape, calculate_size, Size, colours, Aligner, Point};
use haussmann::graphics::colours::RGBA;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

use haussmann::{widgets, Align, Widget, Overflow};
use haussmann::widgets::{Button, Label, Layout, Image, Container};
use haussmann::controllers::tap;

struct Canvas {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    layout: Layout,
}

impl Canvas {
    /// Creates a canvas from a SDL2 window.
    fn from(window: sdl2::video::Window) -> Self {
        Self {
            canvas: window.into_canvas()
                .present_vsync()
                .build()
                .unwrap(),
            layout: Layout::default(),
        }
    }

    /// Changes the draw colour.
    fn change_colour(&mut self, colour: RGBA) {
        self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        self.canvas.set_draw_color(
            Color::RGBA(
                colour.r as u8, 
                colour.g as u8, 
                colour.b as u8, 
                colour.a as u8,
            )
        );
    }

    /// Clears the canvas to white.
    fn clear(&mut self) {
        self.change_colour(RGBA::new(255, 255, 255, 255));
        self.canvas.clear();
    }

    /// Swaps drawing buffer and canvas buffer in order to display the drawings.
    fn present(&mut self) {
        self.canvas.present();
    }
}

impl Drawer for Canvas {
    /// Draws a `shape` on the canvas.
    fn shape(&mut self, shape: &Shape) { 
        let size: Size = calculate_size(shape);

        // If the shape is not filled with a colour, use a transparent colour.
        self.change_colour(
            shape.fill_colour().unwrap_or(colours::TRANSPARENT)
        );

        match shape.points().len() {
            4 => {
                self.canvas.fill_rect(
                    Rect::new(
                        shape.position()[0] as i32, 
                        shape.position()[1] as i32, 
                        size[0] as u32,
                        size[1] as u32,
                    )
                ).unwrap();
            },
            _ => todo!()
        }
    }

    fn image(&mut self, position: Point, _image: &Image) {
        todo!()
    }

    fn label(&mut self, position: Point, label: &Label) {
        // todo
    }

    fn layout(&self) -> &Layout {
        &self.layout
    }
}

#[test]
fn with_sdl2() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    
    // The current size of the window.
    // Updated in the events loop.
    let mut window_size: (i32, i32) = (400, 600);

    let window = video_subsystem.window(
        "Haussmann on SDL2", 
        window_size.0 as u32, 
        window_size.1 as u32
    )
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    // Where to draw the widgets.
    let mut canvas = Canvas::from(window);

    // The layout organizing the widgets on the window.
    let layout = Layout::simple(
        widgets![
            tap::Detector::new(
                Button::simple(
                    Label::simple("Button 1"),
                    RGBA::new(255, 0, 0, 255),
                ),
                || {
                    println!("button1 was tapped!");
                }
            ), 
            Layout::coloured(
                widgets![
                    Container::simple(
                        [200, 100],
                        Button::simple(
                            Label::simple("Button 3"),
                            RGBA::new(0, 0, 255, 255),
                        )
                    )
                ],
                RGBA::new(0, 0, 0, 50),
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
        Overflow::Ignore,
        Align::Center,
        Align::Center,
        Direction::Row,
    );

    canvas.layout = layout;

    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        canvas.clear();

        canvas.draw([0, 0], [window_size.0 as usize, window_size.1 as usize]);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                }
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(width, height) => {
                        window_size = (width, height);
                    }
                    _ => {}
                }
                Event::MouseButtonDown {mouse_btn, x, y, .. } => {
                    if mouse_btn != MouseButton::Left {
                        return;
                    }

                    todo!("on click on clickable widgets")
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
} 
