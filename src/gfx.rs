use std::vec::Vec;

use sdl2::render::{Renderer, Texture, TextureAccess};
use sdl2::pixels::PixelFormatEnum::BGR24;
use sdl2::rect::{Rect};
use sdl2::{Sdl};
use sdl2;

const SCREEN_WIDTH: u32  = 640;
const SCREEN_HEIGHT: u32 = 320;

pub struct Gfx<'a> {
    pub renderer: Renderer<'a>,
    pub texture: Texture,
    pub scale: usize
}

impl<'a> Gfx<'a> {
    pub fn new(scale: usize) -> (Gfx<'a>, Sdl) {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();

        let mut window_builder = video.window("RustyChip8",
                                              (SCREEN_WIDTH as usize * scale) as u32,
                                              (SCREEN_HEIGHT as usize * scale) as u32);

        let window   = window_builder.position_centered().build().unwrap();
        let renderer = window.renderer().accelerated().present_vsync().build().unwrap();

        let texture = renderer.create_texture(BGR24,
                                              TextureAccess::Streaming,
                                              SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();

        (Gfx {
            renderer: renderer,
            texture: texture,
            scale: scale
        }, sdl)
    }

    pub fn composite(&mut self, buffer: Vec<u8>) {
        let mut pixel: u8;

        self.renderer.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));

        for y in 0..32 {
            for x in 0..64 {
                pixel = &buffer[y * 64 + x] * 255;
                self.renderer.fill_rect(
                    Rect::new(x as i32, y as i32, self.scale as u32, self.scale as u32));
            }
        }
    }

    // pub fn composite(&mut self, buffer: Vec<u8>) {
    //     self.blit(buffer);
    //     self.renderer.clear();
    //     self.renderer.copy(&self.texture, None, None);
    //     self.renderer.present();
    // }

    // pub fn blit(&mut self, buffer: Vec<u8>) {
    //     self.texture.update(None, &buffer, SCREEN_WIDTH as usize * 3).unwrap();
    // }
}
