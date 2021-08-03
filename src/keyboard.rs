pub struct Keyboard {
    keys: [State; 0xF],

    key_pressed: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum State{
    PRESSED,
    HELD,
    RELEASED,
    NOTPRESSED,
}

impl Keyboard{
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [State::NOTPRESSED; 0xF],
            key_pressed: false,
        }
    }
    pub fn reset_key_press(&mut self) {
        self.key_pressed = false;
    }

    pub fn process_key (&mut self, key: usize, state: State) {
        if state == State::PRESSED {
            self.key_pressed = true;
        }

        if self.keys[key] == State::PRESSED || self.keys[key] == State::HELD {
            if state == State::PRESSED {
                self.keys[key] = State::HELD;
            } else {
                self.keys[key] = State::RELEASED;
            }
        } else {
            self.keys[key] = state; // No need to have specific transitions
        }
    }

    pub fn was_any_key_pressed(&mut self) -> bool {
        self.key_pressed
    }

    pub fn is_key_pressed(&mut self, key: usize) -> bool {
        self.keys[key] == State::PRESSED
    }
}
