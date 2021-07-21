pub struct Keyboard {
    keys: [State; 0xF],
}

pub enum State{
    PRESSED,
    HELD,
    RELEASED,
    NOT_PRESSED
}

impl Keyboard{

    fn process_key (&mut self, key: u8, State: State) {
        if self.keys[key] == State::PRESSED || self.keys[key] == State::HELD {
            if State == State::PRESSED {
                self.keys[key] = State::HELD;
            } else {
                self.keys[key] = State::RELEASED;
            }
        } else {
            self.keys[key] = State; // No need to have specific transitions
        }

        /*match key {
          A => {},
          B => {},
          C => {},
          D => {},
          E => {},
          F => {},
          0 => {},
          1 => {},
          2 => {},
          3 => {},
          4 => {},
          5 => {},
          6 => {},
          7 => {},
          8 => {},
          9 => {},
          }*/
    }
}
