use mmu::Mmu;
use std::vec::Vec;
use rand::random;

pub struct Cpu {
    mmu: Mmu,
    registers: Vec<u8>,
    stack: Vec<u16>,
    video: Vec<u8>,
    input: Vec<u8>,
    delay_timer: u8,
    sound_timer: u8,
    sp: u16,
    pc: u16,
    i: u16
}

impl Cpu {
    pub fn new(mmu: Mmu) -> Cpu {
        Cpu {
            mmu: mmu,
            registers: vec![0; 16],
            stack: vec![0; 16],
            video: vec![0; 2048],
            input: vec![0; 16],
            delay_timer: 0,
            sound_timer: 0,
            sp: 0,
            pc: 0x200,
            i: 0
        }
    }

    pub fn step(&mut self) {
        print_address(self.pc);

        let instruction = self.mmu.read_word(self.pc as usize);
        self.execute(instruction);

        println!("");

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP");
            }
            self.sound_timer -= 1;
        }
    }

    fn execute(&mut self, instruction: u16) {
        println!("Instruction: {}", format!("{:X}", instruction));

        match instruction {
            0x00E0 => {
                self.cls();
            },
            0x00EE => {
                self.ret();
            },
            0x1000 ... 0x1FFF => {
                self.jp(instruction);
            },
            0x2000 ... 0x2FFF => {
                self.call(instruction);
            },
            0x3000 ... 0x3FFF => {
                self.se_v(instruction);
            },
            0x4000 ... 0x4FFF => {
                self.sne_v(instruction);
            },
            0x5000 ... 0x5FFF => {
                self.se_v_v(instruction);
            },
            0x6000 ... 0x6FFF => {
                self.ld_v(instruction);
            },
            0x7000 ... 0x7FFF => {
                self.add_v(instruction);
            },
            0x8000 ... 0x8FFF => {
                match instruction & 0x000F {
                    0x0000 => {
                        self.ld_v_v(instruction);
                    },
                    0x0001 => {
                        self.or_v_v(instruction);
                    },
                    0x0002 => {
                        self.and_v_v(instruction);
                    },
                    0x0003 => {
                        self.xor_v_v(instruction);
                    },
                    0x0004 => {
                        self.add_v_v(instruction);
                    },
                    0x0005 => {
                        self.sub_v_v(instruction);
                    },
                    0x0006 => {
                        self.shr_v(instruction);
                    },
                    0x0007 => {
                        self.subn_v_v(instruction);
                    },
                    0x000E => {
                        self.shl_v(instruction);
                    },
                    _ => {
                        missing_instruction(instruction);
                    }
                }
            },
            0x9000 ... 0x9FFF => {
                self.sne_v_v(instruction);
            },
            0xA000 ... 0xAFFF => {
                self.ld_i(instruction);
            },
            0xB000 ... 0xBFFF => {
                self.jp_v0(instruction);
            },
            0xC000 ... 0xCFFF => {
                self.rnd_v(instruction);
            },
            0xD000 ... 0xDFFF => {
                self.drw_vv(instruction);
            },
            0xE000 ... 0xEFFF => {
                match instruction & 0x00FF {
                    0x009E => {
                        self.skp_v(instruction);
                    },
                    0x00A1 => {
                        self.sknp_v(instruction);
                    },
                    _ => {
                        missing_instruction(instruction);
                    }
                }
            },
            0xF000 ... 0xFFFF => {
                match instruction & 0x00FF {
                    0x0007 => {
                        self.ld_v_dt(instruction);
                    },
                    0x000A => {
                        self.ld_v_k(instruction);
                    },
                    0x0015 => {
                        self.ld_dt_v(instruction);
                    },
                    0x0018 => {
                        self.ld_st_v(instruction);
                    },
                    0x001E => {
                        self.add_i_v(instruction);
                    },
                    0x0029 => {
                        self.ld_f_v(instruction);
                    },
                    0x0033 => {
                        self.ld_b_v(instruction);
                    },
                    0x0055 => {
                        self.ld_i_v(instruction);
                    },
                    0x0065 => {
                        self.ld_v_i(instruction);
                    },
                    _ => {
                        missing_instruction(instruction);
                    }
                }
            },
            _ => {
                missing_instruction(instruction);
            }
        }
    }

    // 0x00E0
    fn cls(&mut self) {
        self.video = vec![0; 2048];

        self.pc += 2;
    }

    // 0x00EE
    fn ret(&mut self) {
        self.sp -= 1;
        self.pc = self.stack[self.sp as usize] + 2;
    }

    // 0x1nnn
    fn jp(&mut self, instruction: u16) {
        self.pc = instruction & 0x0FFF;
    }

    // 0x2nnn
    fn call(&mut self, instruction: u16) {
        self.stack[self.sp as usize] = self.pc;
        self.sp += 1;
        self.pc = instruction & 0x0FFF;
    }

    // 0x3xkk
    fn se_v(&mut self, instruction: u16) {
        let (register, value) = register_and_value_from(instruction);

        if self.registers[register] == value {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0x4xkk
    fn sne_v(&mut self, instruction: u16) {
        let (register, value) = register_and_value_from(instruction);

        if self.registers[register] != value {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0x5xy0
    fn se_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        if self.registers[x] == self.registers[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0x6xkk
    fn ld_v(&mut self, instruction: u16) {
        let (register, value) = register_and_value_from(instruction);

        self.registers[register] = value;

        self.pc += 2;
    }

    // 0x7xkk
    fn add_v(&mut self, instruction: u16) {
        let (register, value) = register_and_value_from(instruction);

        let original = self.registers[register];

        self.registers[register] = original.wrapping_add(value);

        if original > self.registers[register] {
           self.registers[0xF] = 1;
        } else {
           self.registers[0xF] = 0;
        }

        self.pc += 2;
    }

    // 0x8xy0
    fn ld_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        self.registers[x] = self.registers[y];

        self.pc += 2;
    }

    // 0x8xy1
    fn or_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        self.registers[x] = self.registers[x] | self.registers[y];

        self.pc += 2;
    }

    // 0x8xy2
    fn and_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        self.registers[x] = self.registers[x] & self.registers[y];

        self.pc += 2;
    }

    // 0x8xy3
    fn xor_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        self.registers[x] = self.registers[x] ^ self.registers[y];

        self.pc += 2;
    }

    // 0x8xy4
    fn add_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        let original = self.registers[x];

        self.registers[x] = original.wrapping_add(self.registers[y]);

        if original > self.registers[x] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.pc += 2;
    }

    // 0x8xy5
    fn sub_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        if self.registers[x] > self.registers[y] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x] = self.registers[x].wrapping_sub(self.registers[y]);

        self.pc += 2;
    }

    // 0x8xy6
    fn shr_v(&mut self, instruction: u16) {
        let (x, value) = register_and_value_from(instruction);

        self.registers[0xF] = value & 0x1;

        self.registers[x] >>= 1;

        self.pc += 2;
    }

    // 0x8xy7
    fn subn_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        if self.registers[y] > self.registers[x] {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }

        self.registers[x] = self.registers[y].wrapping_sub(self.registers[x]);

        self.pc += 2;
    }

    // 0x8xyE
    fn shl_v(&mut self, instruction: u16) {
        let (x, value) = register_and_value_from(instruction);

        self.registers[0xF] = value >> 7;

        self.registers[x] <<= 1;

        self.pc += 2;
    }

    // 0x9xy0
    fn sne_v_v(&mut self, instruction: u16) {
        let (x, y) = registers_from(instruction);

        if self.registers[x] != self.registers[y] {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0xAnnn
    fn ld_i(&mut self, instruction: u16) {
        self.i = instruction & 0x0FFF;
        self.pc += 2;
    }

    // 0xBnnn
    fn jp_v0(&mut self, instruction: u16) {
        self.pc = (instruction & 0x0FFF) + self.registers[0];
    }

    // 0xCxkk
    fn rnd_v(&mut self, instruction: u16) {
        let (register, value) = register_and_value_from(instruction);
        let random_value = random::<u8>();

        self.registers[register] = value & random_value;

        self.pc += 2;
    }

    // 0xDxyn
    fn drw_vv(&mut self, instruction: u16) {
        let vx = ((instruction & 0x0F00) >> 8) as usize;
        let vy = ((instruction & 0x00F0) >> 4) as usize;

        let x = self.registers[vx];
        let y = self.registers[vy];

        let height = instruction & 0x000F;

        let mut pixel: u16;

        self.registers[0xF] = 0;

        for yline in 0..height {

            pixel = self.mmu.read_byte((self.i + yline) as usize) as u16;

            for xline in 0..8 {
                if pixel & (0x80 >> xline) != 0 {
                    let vc = video_coordinates(x, y, xline, yline);

                    if self.video[vc] == 1 {
                        self.registers[0xF] = 1;
                    }
                    self.video[vc] ^= 1;
                }
            }
        }

        self.pc += 2;
    }

    // 0xEx9E
    fn skp_v(&mut self, instruction: u16) {
        let register = register_from(instruction);
        let key      = self.registers[register] as usize;

        if self.input[key] == 1 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0xExA1
    fn sknp_v(&mut self, instruction: u16) {
        let register = register_from(instruction);
        let key      = self.registers[register] as usize;

        if self.input[key] != 1 {
            self.pc += 4;
        } else {
            self.pc += 2;
        }
    }

    // 0xFx07
    fn ld_v_dt(&mut self, instruction: u16) {
        let register = register_from(instruction);

        self.registers[register] = self.delay_timer;

        self.pc += 2;
    }

    // 0xFx0A
    fn ld_v_k(&mut self, instruction: u16) {
        let register = register_from(instruction);

        // get key state

        for (i, key) in self.input.iter().enumerate() {
            if *key == 1 {
                self.registers[register] = i as u8;
                self.pc += 2;
            }
        }
    }

    // 0xFx15
    fn ld_dt_v(&mut self, instruction: u16) {
        let register = register_from(instruction);

        self.delay_timer = self.registers[register];

        self.pc += 2;
    }

    // 0xFx18
    fn ld_st_v(&mut self, instruction: u16) {
        let register = register_from(instruction);

        self.sound_timer = self.registers[register];

        self.pc += 2;
    }

    // 0xFx1E
    fn add_i_v(&mut self, instruction: u16) {
        let register = register_from(instruction);

        self.i += self.registers[register] as u16;

        self.pc += 2;
    }

    // 0xFx29
    fn ld_f_v(&mut self, instruction: u16) {
        let register = register_from(instruction);

        self.i = self.registers[register] as u16 * 5;

        self.pc += 2;
    }

    // 0xFx33
    fn ld_b_v(&mut self, instruction: u16) {
        let register = register_from(instruction);
        let value    = self.registers[register];

        self.mmu.write_byte(self.i as usize, value / 100);
        self.mmu.write_byte(self.i as usize + 1, (value / 10) % 10);
        self.mmu.write_byte(self.i as usize + 2, (value % 100) % 10);

        self.pc += 2;
    }

    // 0xFx55
    fn ld_i_v(&mut self, instruction: u16) {
        let last_register = register_from(instruction);

        for index in 0..(last_register + 1) {
            self.mmu.write_byte(self.i as usize + index, self.registers[index]);
        }

        self.pc += 2;
    }

    // 0xFx65
    fn ld_v_i(&mut self, instruction: u16) {
        let last_register = register_from(instruction);

        for index in 0..(last_register + 1) {
            self.registers[index] = self.mmu.read_byte(self.i as usize + index);
        }

        self.pc += 2;
    }
}

fn video_coordinates(x: u8, y: u8, xline: u16, yline: u16) -> usize {
    (x as u16 + xline + ((y as u16 + yline) * 64)) as usize
}

fn register_from(instruction: u16) -> usize {
    register_and_value_from(instruction).0
}

fn registers_from(instruction: u16) -> (usize, usize) {
    let x = (instruction & 0x0F00) >> 8;
    let y = (instruction & 0x00F0) >> 4;

    (x as usize, y as usize)
}

fn register_and_value_from(instruction: u16) -> (usize, u8) {
    let register = (instruction & 0x0F00) >> 8;
    let value    = instruction & 0x00FF;

    (register as usize, value as u8)
}

fn print_address(address: u16) {
    if address >= 512 {
        println!("Address: {} / ROM Address: {}", format!("{:X}", address), format!("{:X}", address - 512));
    } else {
        println!("Address: {}", format!("{:X}", address));
    }
}

fn missing_instruction(instruction: u16) {
    panic!("Missing instruction: {} (HEX) / {} (DEC)",
        format!("{:#X}", instruction), instruction)
}
