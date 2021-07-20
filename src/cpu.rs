use std::process;
use rand::Rng;

use crate::memory::MemoryMap;

struct CPU {
    SP:u16,
    PC:u16,

    //16 V registers
    V: [u8; 0xF],
    //Single I register
    I: u16,
    //Stack in CHIP-8 is
    //limited to 16 elements
    stack: [u16; 0xF],
    memory: Box<MemoryMap>,
}

enum PCIncrement {
    SINGLE,
    SKIP,
    NONE,
}

impl CPU {
    fn getValFromOpCode(opCode : u16, pos : u8) -> usize {
        (opCode >> (pos * 8) & 0xF) as usize
    }

    fn pushPCtoStack(&mut self) {
        self.memory[self.SP] = self.PC;
        self.SP += 1;
    }

    fn popPCfromStack(&mut self) {
        self.PC = self.memory[self.SP];
        self.SP -= 1;
    }

    fn executeNextInstruction(&mut self) {
        self.PC += 1;
        let opCode = self.memory[self.PC];
        if opCode == 0x00E0 {
            //clear screen
        }
        else if opCode == 0xEE {
            self.popPCfromStack();
        }

        // Most of instructions, beside
        // the above ones can be defined
        // by its first byte.
        let firstByte:u8 = (opCode >> 24) as u8;

        //TODO: This is ugly, remove this
        let reg_2 = self.V[CPU::getValFromOpCode(opCode, 2)];
        let mut incrementType = PCIncrement::SINGLE;

        // I can probably do a expression-oriented style
        // for this here
        // (https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html)
        // where each one is inside a function
        // Another solution would be to have a "function match" array, like on my GB emulator
        // But I avoided it just to do things differently here.
        match firstByte {
            0x1 => self.PC = opCode & 0x0FFF,
            0x2 => {
                self.pushPCtoStack();
                self.PC = opCode & 0x0FFF;
            },
            0x3 => {
                let value = opCode & 0xFF;
                if u16::from(reg_2) == value {
                    incrementType = PCIncrement::SKIP;
                }
            },
            0x4 => {
                let value = (opCode & 0xFF) as u8;
                if reg_2 != value {
                    incrementType = PCIncrement::SKIP;
                }
            },
            0x5 => {
                let lastOctal = opCode & 0xF;
                if lastOctal != 0x0 {
                    CPU::abort();
                }

                let regs = (CPU::getValFromOpCode(opCode, 1),
                            CPU::getValFromOpCode(opCode, 2));
                if self.V[regs.0] == self.V[regs.1] {
                    incrementType = PCIncrement::SKIP;
                }
            },
            0x6 => {
                let reg = CPU::getValFromOpCode(opCode, 2);
                let value = (opCode & 0xFF) as u8;
                self.V[reg] = value;
            },
            0x7 => {
                let reg = CPU::getValFromOpCode(opCode, 2);
                let value = (opCode & 0xFF) as u8;
                self.V[reg] += value;
            },
            0x8 => {
                CPU::executeInstrOp8(&mut self.V, opCode);
            },
            0x9 => {
                let lastOctal = opCode & 0xF;
                if lastOctal != 0x0 {
                    CPU::abort();
                }

                let regs = (CPU::getValFromOpCode(opCode, 1),
                            CPU::getValFromOpCode(opCode, 2));
                if self.V[regs.0] != self.V[regs.1] {
                    incrementType = PCIncrement::SKIP;
                }
            },
            0xA => {
                self.I = opCode & 0xFFF;
            },
            0xB => {
                self.PC = (self.V[0x0] as u16) + (opCode & 0xFFF);
            },
            0xC => {
                let reg = CPU::getValFromOpCode(opCode, 2);
                let mut rng = rand::thread_rng();
                let val:u8 = rng.gen();
                self.V[reg] = val & (opCode & 0xFF) as u8;
            },
            0xD => {
               CPU::renderSpritesXY(self.I, self.memory); 
            },
            _ => {
                CPU::abort();
            }
        }

        // Handle PC increment
        match incrementType {
            PCIncrement::SINGLE => self.PC += 1,
            PCIncrement::SKIP => self.PC += 2,
            PCIncrement::NONE => {},
        } 
    }

    fn renderSpritesXY(I:u16, memory: Box<MemoryMap>) {
    }

    fn abort() {
        println!("Invalid instruction");
        process::abort();
    }

    fn executeInstrOp8(V: &mut [u8; 0xF], opCode:u16) {
        let op8 = CPU::getValFromOpCode(opCode, 0);
        let regs = (CPU::getValFromOpCode(opCode, 1),
        CPU::getValFromOpCode(opCode, 2));
        match op8 {
            0x0 => V[regs.0] = V[regs.1],
            0x1 => V[regs.0] = V[regs.0] | V[regs.1],
            0x2 => V[regs.0] = V[regs.0] & V[regs.1],
            0x3 => V[regs.0] = V[regs.0] ^ V[regs.1],
            0x4 => {
                let tmpSum = (V[regs.0] + V[regs.1]) as u8;
                V[0xF] = (tmpSum > 0xFF) as u8;
                V[regs.0] = tmpSum as u8;
            },
            0x5 => {
                V[0xF] = (V[regs.0] > V[regs.1]) as u8;
                V[regs.0] = V[regs.0] - V[regs.1];
            },
            0x6 => {
                V[0xF] = (V[regs.0] & 0x1) as u8;
                V[regs.0] = V[regs.0] >> 1;
            },
            0x7 => {
                V[0xF] = (V[regs.1] > V[regs.0]) as u8;
                V[regs.0] = V[regs.1] - V[regs.0];
            },
            /* No 0x8..0xC */
            0xE => {
                V[0xF] = ((V[regs.0] >> 7) & 0x1) as u8;
                V[regs.0] = V[regs.0] << 1;
            },
            _ => {
                println!("Invalid instruction");
                process::abort();
            }
        }
    }
}
