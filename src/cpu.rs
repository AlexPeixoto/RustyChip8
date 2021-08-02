extern crate bit_vec;

use std::process;
use rand::Rng;

use crate::bus::Bus;
use crate::busstate::BusState;
use crate::memory::MemoryMap;
use crate::keyboard::Keyboard;

use bit_vec::BitVec;

pub struct CPU {
    SP:u16,
    PC:u16,

    //16 V registers
    V: [u8; 0xF],
    //Single I register
    I: u16,
    //Stack in CHIP-8 is
    //limited to 16 elements
    stack: [u16; 0xF],
}

enum PCIncrement {
    SINGLE,
    SKIP,
    NONE,
}

impl CPU {
    pub fn new() -> CPU {
        CPU{
            SP: 0,
            PC: 0,
            V: [0; 0xF],
            I: 0,
            stack: [0; 0xF],
        }
    }

    fn getValFromOpCode(opCode : u16, pos : u8) -> usize {
        (opCode >> (pos * 8) & 0xF) as usize
    }

    fn pushPCtoStack(&mut self) {
        self.stack[self.SP as usize] = self.PC;
        self.SP += 1;
    }

    fn popPCfromStack(&mut self) {
        self.PC = self.stack[self.SP as usize];
        self.SP -= 1;
    }

    pub fn executeNextInstruction(&mut self, memory: &mut MemoryMap, keyboard: &mut Keyboard, state: &mut BusState) {
        // Opcodes are stored in 2 bytes
        let opCode = (memory[self.PC] << 8 | memory[self.PC + 1]) as u16;
        if opCode == 0x00E0 {
            memory.clear_vram();

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
                let last_octal = opCode & 0xF;
                if last_octal != 0x0 {
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
                let last_octal = opCode & 0xF;
                if last_octal != 0x0 {
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
                self.renderSpritesXY(
                    CPU::getValFromOpCode(opCode, 2),
                    CPU::getValFromOpCode(opCode, 1),
                    CPU::getValFromOpCode(opCode, 0),
                    memory);
            },
            0xE => {
                self.executeInstrOpE(&mut incrementType, opCode, keyboard);
            },
            0xF => {
                self.executeInstrOpF(&mut incrementType, opCode, memory, state);
            },
            _ => {
                CPU::abort();
            }
        }

        // Handle PC increment
        match incrementType {
            PCIncrement::SINGLE => self.PC += 2,
            PCIncrement::SKIP => self.PC += 4,
            PCIncrement::NONE => {},
        } 
    }

    /* Maybe move that into GPU in the future? */
    fn renderSpritesXY(&mut self, X:usize, Y:usize, N:usize, memory: &mut MemoryMap) {
        // Initial position warp, but, if it starts at 63 we dont warp
        // further pixel writes
        let x_pos = (self.V[X] % 64) as usize;
        let y_pos = (self.V[Y] % 32) as usize;
        let height = N;

        self.V[0xF] = 0;

        for y in 0..height {
            let byte = self.V[self.I as usize];
            let pixel_vec = BitVec::from_bytes(&[byte]);
            let target_y = y + y_pos;
            for x in 0..8 {
                let mut bit_goal = false;
                let target_x = x + x_pos;
                // We only warp at the start (we break the loop and
                // avoid warp here.
                if target_x >= 64 {
                    break
                }

                let pixel = pixel_vec[x] as bool;
                let is_set = memory.get_vram(target_x, target_y);

                /* Weird pixel set behavior
                 *
                 * If the current pixel in the sprite row is on and the pixel at 
                 * coordinates X,Y on the screen is also on, turn off the pixel 
                 * and set VF to 1
                 *
                 * Or if the current pixel in the sprite row is on and the screen 
                 * pixel is not, draw the pixel at the X and Y coordinates
                 */
                if pixel {
                    if is_set {
                        self.V[0xF] = 1;
                    } else {
                        bit_goal = true;
                    }
                } else if is_set {
                    bit_goal = true;
                }

                /* We only redraw the screen if there
                 * is a pending update.
                 * While the main code will redraw everything
                 * and we could improve that, this will be
                 * left as it is now.
                 */
                if bit_goal != is_set {
                    memory.pending_screen_update(true);
                }

                /* Set the bit on vram */
                memory.set_vram(target_x, target_y, bit_goal);
            }
        }

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
                let tmpSum = (V[regs.0] + V[regs.1]) as u16;
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

    fn executeInstrOpE(&mut self, incrementType: &mut PCIncrement, opCode:u16, keyboard: &mut Keyboard) {
        let subOpCode = opCode & 0xFF;
        let reg = CPU::getValFromOpCode(opCode, 2);
        match subOpCode {
            // Self Keyboard
            0x9E => {
                if keyboard.isKeyPressed(self.V[reg] as usize) {
                    *incrementType = PCIncrement::SKIP;
                }
            },
            0xA1 => {
                if !keyboard.isKeyPressed(self.V[reg] as usize) {
                    *incrementType = PCIncrement::SKIP;
                }
            },
            _ => {
                CPU::abort();
            }
        }
    }

    fn executeInstrOpF(&mut self, incrementType: &mut PCIncrement, opCode:u16, memory: &mut MemoryMap, state: &mut BusState) {
        let subOpCode = opCode & 0xFF;
        let reg = CPU::getValFromOpCode(opCode, 2);
        match subOpCode {
            0x07 =>  self.V[reg] = state.delay,
            0x0A => state.lock_until_pressed = true,
            0x15 => state.delay = self.V[reg],
            0x18 => state.sound = self.V[reg],
            0x1E => {
                let tmpSum = (u16::from(self.V[reg]) + self.I) as u16;
                self.V[0xF] = (tmpSum > 0xFFF) as u8;
                self.I = tmpSum;
            },
            0x29 => {
                //The opcode contains the memory location for the index of the char
                //Each char has 5 bytes, so we get the position and multiply by 5
                self.I = u16::from(self.V[reg]) * 5;
            },
            0x33 => {
                let memPos = self.V[reg] as usize;
                let mut val = self.V[memPos];
                /*
                 * Run in inverse order
                 * 156 should be stored, for example
                 * as 1, 5, 6 ON [2, 1, 0]
                 */
                for idx in 2..0 {
                    let currentPos = (self.I + idx) as u16;
                    memory[currentPos] = val%10;
                    val = val/10;
                }
            },
            0x55 => {
                /*
                 * The interpreter copies the values of registers V0 through Vx into memory,
                 * starting at the address in I.
                 */
                for idx in 0x0..0xF {
                    let currentPos = (self.I + idx) as u16;
                    memory[currentPos] = self.V[idx as usize];
                }
            },
            0x65 => {
                for idx in 0x0..0xF {
                    let currentPos = (self.I + idx) as u16;
                    self.V[idx as usize] = memory[currentPos];
                }

            },
            _ => {
                CPU::abort();
            }
        }
    }
}
