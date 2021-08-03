pub struct Keyboard {
    keys: [State; 0x10],
    last_pressed_key: u8,
    key_pressed: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State{
    PRESSED,
    RELEASED,
}

impl Keyboard{
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [State::RELEASED; 0x10],
            key_pressed: false,
            last_pressed_key: 0,
        }
    }
    pub fn reset_key_press(&mut self) {
        self.key_pressed = false;
    }

    pub fn process_key (&mut self, key: usize, state: State) {
        if state == State::PRESSED {
            self.key_pressed = true;
        }
        
        self.last_pressed_key = key as u8;
        self.keys[key] = state;
    }

    pub fn was_any_key_pressed(&mut self) -> bool {
        self.key_pressed
    }

    pub fn is_key_pressed(&mut self, key: usize) -> bool {
        self.keys[key] == State::PRESSED
    }

    pub fn get_last_pressed_key(&mut self) -> u8 {
        self.last_pressed_key
    }
}
