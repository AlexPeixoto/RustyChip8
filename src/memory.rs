use std::fs::File;
use std::io::Read;
use std::fs;

struct MemoryMap{
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
    memory: [u16; 0xFFF],
    rom_name: String,
}

impl MemoryMap{
    fn new(rom_name : &str) -> Self
    {
        //this is actually returning a new instance
        Self{memory : [0; 0xFFF], rom_name : rom_name.to_owned()}
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
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ];
        let slice = &mut self.memory[0x50..0x9F];
        for idx in 0..80 {
            self.memory[idx] = font_default[idx];
        }
    }

    fn load_rom(&mut self, filename: &str)
    {
        let slice = &mut self.memory[0x200..0xFFF];
        //Each instruction is 16 bits
        let mut instruction:u16;

        let mut rom = File::open(&filename).expect("Could not open file");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");

        let instructions_count = metadata.len()/2;

        let mut buffer = vec![0; metadata.len() as usize];
        rom.read(&mut buffer);

        //rom.read_exact(&mut slice);
        //
        /*loop {
            let mut chunk:u16;
            file.by_ref().take(chunk as u16);
            if n == 0 { break; }
            slice[pos] = chunk;
        }*/
        //rom.read(&mut slice).expect("buffer overflow");
    }
}