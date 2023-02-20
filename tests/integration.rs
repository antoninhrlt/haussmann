// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{Integrator, Direction};
use haussmann::graphics::{Shape, calculate_size, Size, colours, Aligner};
use haussmann::graphics::colours::RGBA;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

use haussmann::{widgets, Align, Widget, Overflow};
use haussmann::widgets::{Button, Label, Layout, Image};
use haussmann::controllers::tap;

struct DrawableZone {
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl DrawableZone {
    fn from(window: sdl2::video::Window) -> Self {
        Self {
            canvas: window.into_canvas()
                .present_vsync()
                .build()
                .unwrap()
        }
    }

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

    fn clear(&mut self) {
        self.canvas.clear();
    }

    fn present(&mut self) {
        self.canvas.present();
    }

    fn window_colour() -> RGBA {
        RGBA::new(255, 255, 255, 255)
    }
}

impl Integrator for DrawableZone {
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

    fn image(&mut self, image: &Image) {
        //println!("renders image : {:?}", image);
    }

    fn label(&mut self, label: &Label) {
        //println!("renders label : {:?}", label);
    }
}

#[test]
fn with_sdl2() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Haussmann on SDL2", 400, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = DrawableZone::from(window);

    let label = Label::simple("Hello!");

    let button = tap::Detector::new(
        Button::simple(
            [100, 60], 
            RGBA::new(255, 0, 0, 255),
        ),
        || {
            println!("button1 was tapped!");
        }
    );

    let button2 = Button::simple(
        [80, 50], 
        RGBA::new(0, 255, 0, 255),
    );

    let button3 = Button::simple(
        [40, 40], 
        RGBA::new(0, 0, 255, 255),
    );

    let child_layout = Layout::coloured(
        [100, 100],
        widgets![button3],
        RGBA::new(0, 0, 0, 100),
        Overflow::Hide,
        Align::Center,
        Align::Center,
        Direction::Column,
    );

    let mut layout = Layout::fixed_coloured(
        [50, 50],
        [300, 500],
        RGBA::new(0, 0, 0, 100),
        widgets![label, button, child_layout, button2],
        Overflow::Hide, 
        Align::Center, 
        Align::Center,
        Direction::Column,
    );

    let mut event_pump = sdl_context.event_pump().unwrap();
    
    'running: loop {
        canvas.change_colour(DrawableZone::window_colour());
        canvas.clear();

        // Draws the layout and its widgets.
        canvas.layout(&mut layout);
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::MouseButtonDown {
                    mouse_btn, 
                    x, 
                    y, 
                    .. 
                } => {
                    if mouse_btn != MouseButton::Left {
                        return;
                    }

                    let mut aligner = Aligner::new(&layout);

                    for detector in layout.widgets::<tap::Detector>() {
                        let mut widget: Shape = detector.shapes()[0].clone();
                        aligner.align_shape(&mut widget);
                        let position = widget.points()[0];
                        let size = detector.tap_zone();
                        let (x, y) = (x as isize, y as isize);

                        if 
                            (x >= position[0] && x <= position[0] + size[0] as isize) 
                            && (y >= position[1] && y <= position[1] + size[1] as isize)
                        {
                            let callback: fn() = detector.on_tap;
                            callback();
                        }
                    }
                }
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
} 
