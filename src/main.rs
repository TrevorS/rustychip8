extern crate rand;
extern crate sdl2;

mod mmu;
mod cpu;
mod gfx;
mod term_gfx;

use std::time::Duration;
use std::thread;

use mmu::Mmu;
use cpu::Cpu;
use gfx::Gfx;

fn main() {
    let filename = String::from("./roms/UFO");

    let mut mmu = Mmu::new();

    mmu.load_rom(filename);

    let mut cpu = Cpu::new(mmu);

    let (mut gfx, sdl) = Gfx::new(1);

    loop {
        cpu.step();

        gfx.composite(cpu.video_buffer());

        thread::sleep(Duration::new(0, 1200000));
    }
}
