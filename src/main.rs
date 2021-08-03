extern crate sfml;

mod bus;
mod busstate;
mod cpu;
mod memory;
mod keyboard;

use sfml::window::{ContextSettings, Event, Style};
use sfml::graphics::{Color, Image, RenderTarget, RenderWindow, Texture, Sprite};
use crate::bus::Bus;

fn main() {

    let mut window = RenderWindow::new(
        //64 x 32 chip 8 resolution
        (64, 32),
        "RustyChip8 Emulator",
        Style::CLOSE,
        &ContextSettings::default(),
    );
    let mut image = Image::new(64, 32);

    let mut bus = Bus::new("/Users/alexcpeixoto/developer/ibm.ch8");
    let color = Color::rgb(0, 127, 0);
    let black = Color::rgb(9, 0, 0);

    window.set_framerate_limit(60);
    window.set_size((1280, 640));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
        }
        for _tick in 0..4 {
            bus.tick_frame_cpu();
        }
        let screen_updated = bus.was_screen_updated();
        if screen_updated {
            let vram = bus.get_vram();
            for i in 0..64 {
                for j in 0..32 {
                    if vram[(i, j)] {
                        image.set_pixel(i as u32, j as u32, color);
                    } else {
                        image.set_pixel(i as u32, j as u32, black);
                    }
                }
            }
        }
        let texture = Texture::from_image(&image).unwrap();
        let sprite = Sprite::with_texture(&texture);
        window.set_active(true);
        window.draw(&sprite); 
        window.display();

        bus.tick_frame_timer();
    }
}
