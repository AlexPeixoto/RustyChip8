extern crate sfml;

mod bus;
mod busstate;
mod cpu;
mod memory;
mod keyboard;

use keyboard::State;
use sfml::window::{ContextSettings, Event, Style, Key};
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

    let mut bus = Bus::new("./logo.ch8");
    let color = Color::rgb(0, 127, 0);
    let black = Color::rgb(9, 0, 0);

    window.set_framerate_limit(60);
    window.set_size((1280, 640));

    while window.is_open() {
        // Reset key state for this frame
        bus.keyboard.reset_key_press();

        while let Some(event) = window.poll_event() {
            // Request closing for the window
            if event == Event::Closed {
                window.close();
            }
            match event {
                Event::KeyPressed {code, ..} => process_keys(code, &mut bus.keyboard, State::PRESSED),
                Event::KeyReleased{code, ..} => process_keys(code, &mut bus.keyboard, State::RELEASED),
                _ => {},
            };
        }
        for _tick in 0..5 {
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

fn process_keys(key_ev: Key , keyboard: &mut crate::keyboard::Keyboard, state: State) {
    let key_pair = [(Key::Num1, 0x1), (Key::Num2, 0x2), (Key::Num3, 0x3), 
                    (Key::Num4, 0xC), (Key::Q,    0x4), (Key::W,    0x5), 
                    (Key::E,    0x6), (Key::R,    0xD), (Key::A,    0x7), 
                    (Key::S,    0x8), (Key::D,    0x9), (Key::F,    0xE), 
                    (Key::Z,    0xA), (Key::X,    0x0), (Key::C,    0xB), 
                    (Key::V,    0xF)];

    for (key, target) in key_pair.iter() {
        if key_ev == *key {
            keyboard.process_key(*target, state)
        }
    }
}
