use sdl2::rect::Rect;
use sdl2::render::Texture;

use component_derive::Variable;

#[derive(Clone, Variable)]
pub struct Sprite {
    pub tileset: String,
    pub x: i32,
    pub y: i32,
    pub z: f64,
    pub width: u32,
    pub height: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Sprite {
    pub fn new(x: i32, y: i32, z: f64, tileset: &str) -> Sprite {
        Sprite { tileset: tileset.to_string(), x, y, z, flip_x: false, flip_y: false, width: 1, height: 1 }
    }

    pub fn multi(x: i32, y: i32, z: f64, width: u32, height: u32, tileset: &str) -> Sprite {
        Sprite { tileset: tileset.to_string(), x, y, z, width, height, flip_x: false, flip_y: false }
    }

    pub fn sprite(x: i32, y: i32, z: f64, flip_x: bool, flip_y: bool, tileset: &str) -> Sprite {
        Sprite { tileset: tileset.to_string(), x, y, z, flip_x, flip_y, width: 1, height: 1 }
    }
}

pub struct SpriteBatch {
    pub blits: Vec<(Sprite, (i32, i32))>
}

impl SpriteBatch{
    pub fn new() -> Self {
        SpriteBatch { blits: Vec::new() }
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

    pub fn source_rect(&self, Sprite{ x, y, width, height, .. }: &Sprite) -> Rect {
        Rect::new(x * self.tile_width as i32, y * self.tile_height as i32, width * self.tile_width, height * self.tile_height)
    }
}