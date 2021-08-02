#![feature(rustc_private)]
extern crate byteorder;
extern crate bitmatrix;

use std::fs;
use std::fs::File;
use std::io::Read;
use bitmatrix::BitMatrix;

use std::ops::{Index, IndexMut};

pub struct MemoryMap {
    /*
       Memory Map:
       +---------------+= 0xFFF (4095) End of Chip-8 RAM
       |               |
       |               |
       |               |
       |               |
       |               |
       | 0x200 to 0xFFF|
       |     Chip-8    |
       | Program / Data|
       |     Space     |
       |               |
       |               |
       |               |
       +- - - - - - - -+= 0x600 (1536) Start of ETI 660 Chip-8 programs
       |               |
       |               |
       |               |
       +---------------+= 0x200 (512) Start of most Chip-8 programs
       | 0x000 to 0x1FF|
       | Reserved for  |
       |  interpreter  |
       +---------------+= 0x000 (0) Start of Chip-8 RAM
       */
    memory: [u8; 0xFFF],
    rom_name: String,
    vram: BitMatrix,
    vram_changed: bool,
}

impl Index<u16> for MemoryMap {
    type Output = u8;
    fn index<'a>(&'a self, i: u16) -> &'a u8 {
        &self.memory[i as usize]
    }
}

impl IndexMut<u16> for MemoryMap{
    fn index_mut<'a>(&'a mut self, i: u16) -> &'a mut u8 {
        &mut self.memory[i as usize]
    }
}

impl MemoryMap {
    pub fn new(rom_name: &str) -> Self {
        //this is actually returning a new instance
        let mut to_ret = Self {
            memory: [0; 0xFFF],
            //To preserve X, Y, "order" the matrix
            //is "inverted", just to avoid confusion later
            vram: BitMatrix::new(64, 42),
            vram_changed: false,
            rom_name: rom_name.to_owned(),
        };

        for i in 0..32 {
            for j in 0..64 {
                to_ret.vram.set((i, j), false);
            }
        }

        to_ret
    }

    fn init_font(&mut self) {
        let font_default = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        for idx in 0..80 {
            self.memory[idx] = font_default[idx];
        }
    }

    fn load_rom(&mut self, filename: &str) {
        let slice = &mut self.memory[0x200..0xFFF];

        let mut rom = File::open(&filename).expect("Could not open file");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");

        //let instructions_count = metadata.len()/2;

        rom.read(&mut slice[..]);
        //rom.read(&mut slice[..]).unwrap();
    }

    pub fn get_vram(&mut self, x: usize, y: usize) -> bool {
        self.vram[(x, y)]
    }

    pub fn get_full_vram(&self) -> BitMatrix {
        self.vram.clone()
    }

    pub fn set_vram(&mut self, x: usize, y: usize, set: bool) {
        self.vram.set((x, y), set);
    }

    pub fn pending_screen_update(&mut self, updated: bool) {
        self.vram_changed = updated;
    }

    pub fn was_screen_updated(&self) -> bool {
        self.vram_changed.clone()
    }

    pub fn clear_vram(&mut self) {
        for i in 0..32 {
            for j in 0..64 {
                self.vram.set((i, j), false);
            }
        }
        self.vram_changed = true;
    }
}
