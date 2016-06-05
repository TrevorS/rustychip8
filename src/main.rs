extern crate rand;

mod mmu;
mod cpu;

use mmu::Mmu;
use cpu::Cpu;

fn main() {
    println!("RustyChip8 v0.0.1");

    let filename = String::from("./roms/INVADERS");

    let mut mmu = Mmu::new();

    mmu.load_rom(filename);

    let mut cpu = Cpu::new(mmu);

    loop {
        cpu.step();
    }
}
