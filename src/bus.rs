use crate::cpu::CPU;
use crate::gpu::GPU;
use crate::memory::MemoryMap;
use crate::keyboard::Keyboard; 

/* This is a entirelly public
 * struct so other components can have
 * intercomunication via the bus.
 * While this is probably not the best pattern
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

    DT: u8,
    ST: u8,

    lockUntilPressed: bool,
}

impl Bus{
    pub fn new(file: &str) -> Bus {
        Bus {
            cpu: CPU::new(),
            memory: MemoryMap::new(file),
            keyboard: Keyboard::new(),
            DT: 0,
            ST: 0,
            lockUntilPressed: false,
        }
    }
    fn tickFrameTimer(&mut self) {
        if self.DT > 0 {
            self.DT = self.DT - 1;
        }
        if self.ST > 0 {
            self.ST = self.ST - 1;
        }
    }

    fn tickFrameCPUGPU(&mut self) {
        /*if self.keyboard.isAnykeyPressed() && self.lockUntilPressed {
            self.lockUntilPressed = false;
        }
                 
        if self.lockUntilPressed {
            return;
        }*/
        //for clocks per frame
        // self.cpu.tick();
        //self.gpu.tick();
    } 

    fn tickFrame(&mut self) {
        self.tickFrameCPUGPU();
        self.tickFrameTimer();
    }

    pub fn getDT(&mut self) -> u8 {
        self.DT
    }

    pub fn getST(&mut self) -> u8 {
        self.ST
    }

    pub fn setDT(&mut self, val: u8) {
        self.DT = val;
    }

    pub fn setST(&mut self, val: u8) {
        self.ST = val;
    }

    pub fn lockUntilPressed(&mut self) {
        self.lockUntilPressed = true;
    }
}


