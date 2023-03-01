use haussmann::{Overflow, Align, Direction, Widget};
use haussmann::widgets::{View, Layout, Label, Button, Container};
use haussmann::controllers::tap;
use haussmann::graphics::colours::RGBA;
use haussmann::{view, layout, widgets, rgba, button, container, label};

use rand::Rng;

#[test]
fn macros() {
    let view = view! {
        position: [0, 0],
        size: [1280, 720],
        layout! {
            colour: rgba!(255, 255, 255, a: 255),
            overflow: Ignore,
            wx: Align::Center,
            wy: Align::Center,
            direction: Column,
            widgets! [
                button! {
                    colour: rgba!(255, 0, 0, a: 255),
                    label: label!("red button"), 
                    on_tap: |button| {
                        let mut rng = rand::thread_rng();
                        
                        let r = rng.gen_range(0..255);
                        let g = rng.gen_range(0..255);
                        let b = rng.gen_range(0..255);
                        
                        button.colour = rgba!(r, g, b, a: 255);
                    }
                },
                container! {
                    size: [100, 100],
                    widget: button! {
                        colour: rgba!(255, 0, 0, a: 255),
                        label: label!("red button"),
                    }
                }
            ]
        }
    };

    println!("{:#?}", view);
}
