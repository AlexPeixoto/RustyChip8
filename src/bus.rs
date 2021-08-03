extern crate bitmatrix;

use crate::busstate::BusState;
use crate::cpu::CPU;
use crate::memory::MemoryMap;
use crate::keyboard::Keyboard; 
use bitmatrix::BitMatrix;

/* This is a entirelly public
 * state.soundruct so other components can have
 * intercomunication via the bus.
 * While this is probably not the bestate.sound pattern
 * hardware-wise it would be expected that those components
 * can directly communicate, so I avoided creating get*** for that
 * Memory and keyboard are public as the CPU can directlu
 * access it and the GPU can read from it.
 * Nobody can access the CPU but the bus via tick, the same for the GPU.
 */
pub struct Bus{
    cpu: CPU,
    pub memory: MemoryMap,
    pub keyboard: Keyboard,

    pub state: BusState,
}

impl Bus{
    pub fn new(file: &str) -> Bus {
        Bus {
            cpu: CPU::new(),
            memory: MemoryMap::new(file),
            keyboard: Keyboard::new(),
            state: BusState {
                delay: 0,
                sound: 0,
                lock_until_pressed: false,
                write_to: 0x0,
            },
        }
    }

    pub fn was_screen_updated(&self) -> bool {
        self.memory.was_screen_updated()
    }

    pub fn get_vram(&self) -> BitMatrix {
        self.memory.get_full_vram()
    }

    pub fn tick_frame_timer(&mut self) {
        if self.state.delay > 0 {
            self.state.delay = self.state.delay - 1;
        }
        if self.state.sound > 0 {
            self.state.sound = self.state.sound - 1;
        }
    }

    pub fn tick_frame_cpu(&mut self) {
        if self.keyboard.was_any_key_pressed() && self.state.lock_until_pressed {
            self.state.lock_until_pressed = false;

            let last_key_pressed = self.keyboard.get_last_pressed_key();
            self.cpu.write_key_to(self.state.write_to as usize, last_key_pressed);
        }
                    
        if self.state.lock_until_pressed {
            return;
        }
        //four clocks per frame
        //This looks ugly, needs to revisit this once I learn more about the language
        self.cpu.execute_next_instruction(
            &mut self.memory,
            &mut self.keyboard,
            &mut self.state
        );
    } 
}


