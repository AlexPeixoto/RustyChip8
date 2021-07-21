extern crate sfml;

mod bus;
mod cpu;
mod gpu; 
mod memory;
mod keyboard;

/*use sfml::{
    audio::{Sound, SoundBuffer},
    graphics::{
        CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
        Transformable,
    },
    system::{Clock, Time, Vector2f},
    window::{ContextSettings, Event, Key, Style},
};*/
use sfml::window::{ContextSettings, Event, Style};
use sfml::graphics::RenderWindow;

fn main() {
    let mut window = RenderWindow::new(
        //64 x 64 chip 8 resolution
        (640, 640),
        "RustyChip8 Emulator",
        Style::CLOSE,
        &ContextSettings::default(),
    );

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
        }

        //window.set_active(true);
        window.display();
    }

    /*let mut window = match RenderWindow::new(VideoMode::new_init(800, 600, 32),
                                             
                                             Style::CLOSE,
                                             &ContextSettings::default()) {
        Some(window) => window,
        None => panic!("Cannot create a new Render Window.")
    };*/
}
