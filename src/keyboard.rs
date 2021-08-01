pub struct Keyboard {
    keys: [State; 0xF],

    keyPressed: bool,
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
            keyPressed: false,
        }
    }
    pub fn resetKeyPress(&mut self) {
        self.keyPressed = false;
    }

    pub fn process_key (&mut self, key: usize, state: State) {
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

    pub fn isKeyPressed(&mut self, key: usize) -> bool {
        self.keys[key] == State::PRESSED
    }
}
