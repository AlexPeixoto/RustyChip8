extern crate bit_vec;

use std::process;
use rand::Rng;

use crate::busstate::BusState;
use crate::memory::MemoryMap;
use crate::keyboard::Keyboard;

use bit_vec::BitVec;

pub struct CPU {
    sp:u16,
    pc:u16,

    //16 V registers (The doc is confusing
    //it states 0..F and talks about VF, so its 17
    //to make things easier I will create 17 elements
    //instead of 16
    v: [u8; 0x10],
    //Single I register
    i: u16,
    //Stack in CHIP-8 is
    //limited to 16 elements
    stack: [u16; 0x10],
}

enum PcIncrement {
    SINGLE,
    SKIP,
    NONE,
}

impl CPU {
    pub fn new() -> CPU {
        CPU{
            sp: 0,
            pc: 0x200,
            v: [0; 0x10],
            i: 0,
            stack: [0; 0x10],
        }
    }

    fn get_val_from_opcode(opcode : u16, pos : u8) -> usize {
        (opcode >> (pos * 4) & 0xF) as usize
    }

    fn push_pc_to_stack(&mut self) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
    }

    fn pop_pc_from_stack(&mut self) {
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    pub fn execute_next_instruction(&mut self, memory: &mut MemoryMap, keyboard: &mut Keyboard, state: &mut BusState) {
        // Opcodes are stored in 2 bytes
        let shifted_pc:u16 = (memory[self.pc] as u16) << 8;
        let opcode = shifted_pc | (memory[self.pc + 1] as u16);
        println!("{:#06x}", opcode); 
        if opcode == 0x00E0 {
            memory.clear_vram();

        }
        else if opcode == 0xEE {
            self.pop_pc_from_stack();
            /* We let it continue as we want to increment it to get
             * the next instruction, remember that we stored PC at the time
             * of the push
             */
        }

        // Most of instructions, beside
        // the above ones can be defined
        // by its first byte.
        let first_nibble:u8 = (opcode >> 12) as u8;
        /* Its quite common to have the 2nd nibble as
         * a V[x], so to make the code cleaner I do it here
         */
        let regs = (CPU::get_val_from_opcode(opcode, 0),
                    CPU::get_val_from_opcode(opcode, 1),
                    CPU::get_val_from_opcode(opcode, 2));

        let mut increment_type = PcIncrement::SINGLE;

        // I can probably do a expression-oriented style
        // for this here
        // (https://blog.rust-lang.org/2015/04/17/Enums-match-mutation-and-moves.html)
        // where each one is inside a function
        // Another solution would be to have a "function match" array, like on my GB emulator
        // But I avoided it just to do things differently here.
        match first_nibble {
            0x1 => {
                self.pc = opcode & 0x0FFF;
                increment_type = PcIncrement::NONE;
            }
            0x2 => {
                self.push_pc_to_stack();
                self.pc = opcode & 0x0FFF;
                increment_type = PcIncrement::NONE;
            },
            0x3 => {
                let value = opcode & 0xFF;
                if u16::from(self.v[regs.2]) == value {
                    increment_type = PcIncrement::SKIP;
                }
            },
            0x4 => {
                let value = (opcode & 0xFF) as u8;
                if self.v[regs.2] != value {
                    increment_type = PcIncrement::SKIP;
                }
            },
            0x5 => {
                let last_octal = opcode & 0xF;
                if last_octal != 0x0 {
                    CPU::abort();
                }

                if self.v[regs.1] == self.v[regs.2] {
                    increment_type = PcIncrement::SKIP;
                }
            },
            0x6 => {
                let value = (opcode & 0xFF) as u8;
                self.v[regs.2] = value;
            },
            0x7 => {
                let value = opcode as u8;
                self.v[regs.2] = self.v[regs.2].wrapping_add(value);
            },
            0x8 => {
                CPU::execute_instr_op_8(&mut self.v, opcode);
            },
            0x9 => {
                let last_octal = opcode & 0xF;
                if last_octal != 0x0 {
                    CPU::abort();
                }

                if self.v[regs.1] != self.v[regs.2] {
                    increment_type = PcIncrement::SKIP;
                }
            },
            0xA => {
                self.i = opcode & 0xFFF;
            },
            0xB => {
                self.pc = (self.v[0x0] as u16) + (opcode & 0xFFF);
                return;
            },
            0xC => {
                let mut rng = rand::thread_rng();
                let val:u8 = rng.gen();
                self.v[regs.2] = val & (opcode & 0xFF) as u8;
            },
            0xD => {
                self.render_sprites_x_y(regs.2, regs.1, regs.0, memory);
            },
            0xE => {
                self.execute_instr_op_e(&mut increment_type, opcode, keyboard);
            },
            0xF => {
                self.execute_instr_op_f(opcode, memory, state);
            },
            _ => {
            }
        }

        // Handle pc increment
        match increment_type {
            PcIncrement::SINGLE => self.pc += 2,
            PcIncrement::SKIP => self.pc += 4,
            PcIncrement::NONE => {},
        } 
    }

    /* Maybe move that into GPU in the future? */
    fn render_sprites_x_y(&mut self, x:usize, y:usize, n:usize, memory: &mut MemoryMap) {
        // Initial position warp, but, if it starts at 63 we dont warp
        // further pixel writes
        let x_pos = (self.v[x] % 64) as usize;
        let y_pos = (self.v[y] % 32) as usize;
        let height = n;

        self.v[0xF] = 0;

        for y in 0..height {
            let byte = memory[self.i + y as u16];
            //println!("byte pos: {:#06x}", self.i); 
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
                        self.v[0xF] = 1;
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

    fn execute_instr_op_8(v: &mut [u8; 0x10], opcode:u16) {
        let op8 = CPU::get_val_from_opcode(opcode, 0);
        let regs = (CPU::get_val_from_opcode(opcode, 1),
        CPU::get_val_from_opcode(opcode, 2));
        match op8 {
            0x0 => v[regs.0] = v[regs.1],
            0x1 => v[regs.0] = v[regs.0] | v[regs.1],
            0x2 => v[regs.0] = v[regs.0] & v[regs.1],
            0x3 => v[regs.0] = v[regs.0] ^ v[regs.1],
            0x4 => {
                let tmp_sum = (v[regs.0] + v[regs.1]) as u16;
                v[0xF] = (tmp_sum > 0xFF) as u8;
                v[regs.0] = tmp_sum as u8;
            },
            0x5 => {
                v[0xF] = (v[regs.0] > v[regs.1]) as u8;
                v[regs.0] = v[regs.0] - v[regs.1];
            },
            0x6 => {
                v[0xF] = (v[regs.0] & 0x1) as u8;
                v[regs.0] = v[regs.0] >> 1;
            },
            0x7 => {
                v[0xF] = (v[regs.1] > v[regs.0]) as u8;
                v[regs.0] = v[regs.1] - v[regs.0];
            },
            /* No 0x8..0xC */
            0xE => {
                v[0xF] = ((v[regs.0] >> 7) & 0x1) as u8;
                v[regs.0] = v[regs.0] << 1;
            },
            _ => {
                println!("Invalid instruction");
                process::abort();
            }
        }
    }

    fn execute_instr_op_e(&mut self, increment_type: &mut PcIncrement, opcode:u16, keyboard: &mut Keyboard) {
        let sub_op_code = opcode & 0xFF;
        let reg = CPU::get_val_from_opcode(opcode, 2);
        match sub_op_code {
            // Self Keyboard
            0x9E => {
                if keyboard.is_key_pressed(self.v[reg] as usize) {
                    *increment_type = PcIncrement::SKIP;
                }
            },
            0xA1 => {
                if !keyboard.is_key_pressed(self.v[reg] as usize) {
                    *increment_type = PcIncrement::SKIP;
                }
            },
            _ => {
                CPU::abort();
            }
        }
    }

    fn execute_instr_op_f(&mut self, opcode:u16, memory: &mut MemoryMap, state: &mut BusState) {
        let sub_op_code = opcode & 0xFF;
        let reg = CPU::get_val_from_opcode(opcode, 2);
        match sub_op_code {
            0x07 =>  self.v[reg] = state.delay,
            0x0A => state.lock_until_pressed = true,
            0x15 => state.delay = self.v[reg],
            0x18 => state.sound = self.v[reg],
            0x1E => {
                let tmp_sum = (u16::from(self.v[reg]) + self.i) as u16;
                self.v[0xF] = (tmp_sum > 0xFFF) as u8;
                self.i = tmp_sum;
            },
            0x29 => {
                //The opcode contains the memory location for the index of the char
                //Each char has 5 bytes, so we get the position and multiply by 5
                self.i = u16::from(self.v[reg]) * 5;
            },
            0x33 => {
                let mut val = self.v[reg];
                /*
                 * Run in inverse order
                 * 156 should be stored, for example
                 * as 1, 5, 6 ON [2, 1, 0]
                 */
                for idx in (0..3).rev() {
                    let current_pos = (self.i + idx) as u16;
                    memory[current_pos] = val%10;
                    val = val/10;
                }
            },
            0x55 => {
                /*
                 * The interpreter copies the values of registers V0 through Vx into memory,
                 * starting at the address in I.
                 */
                for idx in 0x0..0xF {
                    let current_pos = (self.i + idx) as u16;
                    memory[current_pos] = self.v[idx as usize];
                }
            },
            0x65 => {
                for idx in 0x0..0xF {
                    let current_pos = (self.i + idx) as u16;
                    self.v[idx as usize] = memory[current_pos];
                }

            },
            _ => {
                CPU::abort();
            }
        }
    }
}
