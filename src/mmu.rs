use std::vec::Vec;
use std::io::Error;
use std::io::Read;
use std::fs::File;

pub struct Mmu {
    memory: Vec<u8>,
    fontset: Vec<u8>
}

impl Mmu {
    pub fn new() -> Mmu {
        let mut mmu = Mmu {
            memory:  vec![0; 4096],
            fontset: vec![
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
            ]
        };

        mmu.reset();

        mmu
    }

    pub fn reset(&mut self) {
        self.memory = vec![0; 4096];
        for (i, value) in self.fontset.iter().enumerate() {
            self.memory[i] = *value;
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        self.memory[address] = value;
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        self.memory[address]
    }

    pub fn read_word(&self, address: usize) -> u16 {
        (self.read_byte(address) as u16) << 8 | (self.read_byte(address + 1) as u16)
    }

    pub fn load_rom(&mut self, filename: String) {
        // println!("Loading ROM: #{}", filename);
        match read_rom(filename) {
            Ok(rom) => {
                for (i, value) in rom.iter().enumerate() {
                    self.memory[i + 512] = *value;
                }
            },
            Err(e) => {
                panic!("Error: {}", e);
            }
        }
    }
}

fn read_rom(filename: String) -> Result<Vec<u8>, Error> {
    let mut file   = try!(File::open(filename));
    let mut buffer = Vec::new();

    try!(file.read_to_end(&mut buffer));

    Ok(buffer)
}
