// This file is part of "haussmann"
// Under the MIT License
// Copyright (c) 2023 Antonin Hérault

use haussmann::{Integrator, widgets::{Layout, Image, Label, Container, Surface}, Overflow, Align, Direction, widgets, graphics::{Drawer, Shape, Point, Size, calculate_size, colours::{self, RGBA}}, Widget};
use sdl2;

struct App {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
    event_pump: sdl2::EventPump,
}

impl App {
    fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
    
        let window = video_subsystem
            .window("Haussmann on SDL2", 600, 500)
            .position_centered()
            .build()
            .unwrap();

        Self {
            canvas: window.into_canvas().present_vsync().build().unwrap(),
            event_pump: sdl_context.event_pump().unwrap(),
        }
    }
}

impl Integrator for App{
    fn view() -> Layout {
        Layout::simple(
            widgets![
                Container::coloured(
                    [100, 100],
                    colours::RED,
                    Surface::coloured(colours::TRANSPARENT),
                )
            ], 
            Overflow::Ignore, 
            Align::Center, 
            Align::Center, 
            Direction::Column,
        )
    }

    fn run(&mut self) {
        'running: loop {
            self.clear(RGBA::new(100, 0, 0, 255));

            self.draw([0, 0], [600, 500]);

            for event in self.event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit { .. } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            self.canvas.present();
        }
    }

    fn clear(&mut self, colour: RGBA) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
            colour.r as u8,
            colour.g as u8,
            colour.b as u8,
            colour.a as u8,
        ));

        self.canvas.clear();
    }
}

impl Drawer for App{
    fn image(&mut self, _position: Point, _image: &Image) {
        
    }

    fn label(&mut self, _position: Point, _label: &Label) {
        
    }

    fn shape(&mut self, shape: &Shape) {
        let size: Size = calculate_size(shape);

        let colour = shape.fill_colour().unwrap_or(colours::TRANSPARENT);

        self.canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        self.canvas.set_draw_color(sdl2::pixels::Color::RGBA(
            colour.r as u8,
            colour.g as u8,
            colour.b as u8,
            colour.a as u8,
        ));

        match shape.points().len() {
            4 => {
                self.canvas
                    .fill_rect(sdl2::rect::Rect::new(
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

    fn layout(&self) -> Layout {
        Self::view()
    }
}

#[test]
fn run_app() {
    let mut app = App::new();
    app.run();
}
