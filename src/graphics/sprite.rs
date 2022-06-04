use sdl2::rect::Rect;
use sdl2::render::Texture;

use component_derive::Variable;
use entity::{ Component, Variable };

#[derive(Copy, Clone, Variable)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub const fn new(x: i32, y: i32) -> Sprite {
        Sprite { x, y, flip_x: false, flip_y: false, width: 1, height: 1 }
    }

    pub const fn multi(x: i32, y: i32, width: u32, height: u32) -> Sprite {
        Sprite { x, y, width, height, flip_x: false, flip_y: false }
    }

    pub const fn sprite(x: i32, y: i32, flip_x: bool, flip_y: bool) -> Sprite {
        Sprite { x, y, flip_x, flip_y, width: 1, height: 1 }
    }
}

pub struct SpriteBatch<'a> {
    pub spritesheet: &'a Texture<'a>,
    pub tile_width: u32,
    pub tile_height: u32,
    pub blits: Vec<(Sprite, (i32, i32))>
}

impl <'a> SpriteBatch<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, tile_width: u32, tile_height: u32) -> Self {
        SpriteBatch { spritesheet, tile_width, tile_height, blits: Vec::new() }
    }

    pub fn source_rect(&self, Sprite{ x, y, width, height, .. }: Sprite) -> Rect {
        Rect::new(x * self.tile_width as i32, y * self.tile_height as i32, width * self.tile_width, height * self.tile_height)
    }

    pub fn blit(&mut self, source: Sprite, x: f64, y: f64) {
        let x = x.round() as i32;
        let y = y.round() as i32;
        self.blits.push((source, (x, y)));
    }
}

pub struct SpriteSheet<'a> {
    pub spritesheet: &'a Texture<'a>,
    pub tile_width: u32,
    pub tile_height: u32,
}

impl <'a> SpriteSheet<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, tile_width: u32, tile_height: u32) -> Self {
        SpriteSheet { spritesheet, tile_width, tile_height }
    }

    pub fn batch(&self) -> SpriteBatch<'a> {
        SpriteBatch::new(&self.spritesheet, self.tile_width, self.tile_height)
    }
}