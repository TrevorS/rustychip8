use cpu::Cpu;

pub struct TermGfx { }

impl TermGfx {
    pub fn new() -> TermGfx {
        TermGfx { }
    }

    pub fn composite(&self, buffer: Vec<u8>) {
        for row in buffer.chunks(64) {
            println!("{}", "");
            for byte in row.iter() {
                match *byte {
                    0x0 => { print!("{}", " ") },
                    _   => { print!("{}", "X") }
                }
            }
        }
        print!("{}[2J", 27 as char);
    }
}
