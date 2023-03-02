use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use haussmann::{widgets::{Container, Label, View, Layout, Surface, Button}, Overflow, Direction, container, label, view, layout, Align, widgets, graphics::draw::Drawable, Widget, rgba, graphics::colours::RGBA, surface, button};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    buffer::Buffer,
    layout::Rect,
    style::{Style, Modifier, Color},
    widgets::Widget as TuiWidget,
    Frame, Terminal,
};

#[test]
fn tui()  {
    fn main() -> Result<(), Box<dyn Error>> {
        // setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // create app and run it
        let res = run_app(&mut terminal);

        // restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        if let Err(err) = res {
            println!("{:?}", err)
        }

        Ok(())
    }

    main().ok();
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

struct TuiView {
    view: View,
}

impl TuiWidget for TuiView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let drawables = self.view.build();
        for drawable in drawables {
            match drawable {
                Drawable::Label(label, position, size) => {
                    buf.set_string(position[0] as u16, position[1] as u16, label.text, Style::default());
                }
                Drawable::Surface(surface, position, size) => {
                    let rect = Rect {
                        x: position[0] as u16,
                        y: position[0] as u16,
                        width: size[0] as u16,
                        height: size[1] as u16,
                    };

                    let style = Style { 
                        fg: None, 
                        bg: Some(Color::Rgb(surface.colour.r as u8, surface.colour.g as u8, surface.colour.b as u8)), 
                        add_modifier: Modifier::BOLD, 
                        sub_modifier: Modifier::BOLD, 
                    };

                    buf.set_style(rect, style)
                }
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let size = f.size();

    let container = TuiView {
        view: view! {
            position: [0, 0],
            size: [40, 20],
            layout! {
                colour: rgba!(255, 255, 255, a: 255),
                overflow: Ignore,
                wx: Align::Center,
                wy: Align::Center,
                direction: Column,
                widgets![
                    surface! {
                        colour: rgba!(255, 0, 0, a: 255),
                    },
                    container! {
                        size: [10, 5],
                        widget: button! {
                            colour: rgba!(0, 255, 0, a: 255),
                            label: label!("haussmann on tui!"),
                        },
                    },
                ]
            }
        }
    };

    f.render_widget(container, size);
}
