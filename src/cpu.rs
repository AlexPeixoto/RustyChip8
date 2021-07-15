use std::process;

use crate::memory::MemoryMap;

struct CPU {
    SP:u16,
    PC:u16,

    //16 V registers
    V: [u8; 0xF],
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

    fn executeNextInstruction(&mut self) {
        self.PC += 1;
        let opCode = self.memory[self.PC];
        if opCode == 0x00E0 {
            //clear screen
        }
        else if opCode == 0xEE {
            self.PC = self.memory[self.SP];
            self.SP -= 1;
        }

        // Most of instructions, beside
        // the above ones can be defined
        // by its first byte.
        let firstByte:u8 = opCode >> 24;

        let reg_2 = self.V[CPU::getValFromOpCode(opCode, 2)];
        let mut extraInc = 0;
        let mut incrementType = PCIncrement::SINGLE;

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
                let regs = (CPU::getValFromOpCode(opCode, 1),
                            CPU::getValFromOpCode(opCode, 2));
                if self.V[regs.0] == self.V[regs.1] {
                    extraInc = 1;
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
                self.executeInstrOp8(opCode);
            },
            _ => {
                println!("Invalid instruction");
                process::abort();
            }
        }
        match incrementType {
            SINGLE => self.PC += 1,
            SKIP => self.PC += 2,
        } 
    }
    fn executeInstrOp8(&mut self, opCode:u16) {
        let op8 = CPU::getValFromOpCode(opCode, 0);
        let regs = (CPU::getValFromOpCode(opCode, 1),
        CPU::getValFromOpCode(opCode, 2));
        match op8 {
            0x0 => self.V[regs.0] = self.V[regs.1],
            0x1 => self.V[regs.0] = self.V[regs.0] | self.V[regs.1],
            0x2 => self.V[regs.0] = self.V[regs.0] & self.V[regs.1],
            0x3 => self.V[regs.0] = self.V[regs.0] ^ self.V[regs.1],
            0x4 => {
                let tmpSum = (self.V[regs.0] + self.V[regs.1]) as u8;
                self.V[0xF] = (tmpSum > 0xFF) as u8;
                self.V[regs.0] = tmpSum as u8;
            },
            _ => {
                println!("Invalid instruction");
                process::abort();
            }
        }
    }
}
