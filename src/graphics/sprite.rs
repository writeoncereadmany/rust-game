use sdl2::rect::Rect;
use sdl2::render::Texture;

pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub flip_x: bool,
    pub flip_y: bool,
}

pub struct SpriteBatch<'a> {
    pub spritesheet: &'a Texture<'a>,
    pub blits: Vec<(Rect, (i32, i32), (bool, bool))>
}

impl <'a> SpriteBatch<'a> {
    pub fn new(spritesheet: &'a Texture<'a>) -> Self {
        SpriteBatch { spritesheet, blits: Vec::new() }
    }

    pub fn blit(&mut self, source: Rect, x: f64, y: f64, flip_x: bool, flip_y: bool) {
        let x = x.round() as i32;
        let y = y.round() as i32;
        self.blits.push((source, (x, y), (flip_x, flip_y)));
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

    pub fn tile2(&self, x: i32, y: i32, width: u32, height: u32) -> Rect {
        Rect::new(x * self.tile_width as i32, y * self.tile_height as i32, width * self.tile_width, height * self.tile_height)
    }

    pub fn batch(&self) -> SpriteBatch<'a> {
        SpriteBatch::new(&self.spritesheet)
    }
}