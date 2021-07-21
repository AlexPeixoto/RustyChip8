pub struct Keyboard {
    keys: [State; 0xF],

    keyPressed: bool,
}

pub enum State{
    PRESSED,
    HELD,
    RELEASED,
    NOT_PRESSED
}

impl Keyboard{

    pub fn resetKeyPress(&mut self) {
        self.keyPressed = false;
    }

    pub fn process_key (&mut self, key: u8, state: State) {
        if state == State::PRESSED {
            self.keyPressed = true;
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

    pub fn isAnyKeyPressed(&mut self) -> bool {
        self.keyPressed
    }
}
