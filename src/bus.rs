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
    gpu: GPU,
    pub memory: MemoryMap,
    pub keyboard: Keyboard,

    frameTimerCount: u16,
}

impl Bus{
}


