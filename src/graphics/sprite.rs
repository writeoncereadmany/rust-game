use sdl2::rect::Rect;
use sdl2::render::Texture;

#[derive(Clone)]
pub struct Sprite<'a> {
    pub spritesheet: &'a Texture<'a>,
    pub source_rect: Rect
}

impl <'a> Sprite<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, source_rect: Rect) -> Self {
        Sprite{
            spritesheet,
            source_rect
        }
    }
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
    spritesheet: &'a Texture<'a>,
    pub tile_width: u32,
    pub tile_height: u32,
}

impl <'a> SpriteSheet<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, tile_width: u32, tile_height: u32) -> Self {
        SpriteSheet { spritesheet, tile_width, tile_height }
    }

    pub fn sprite(&self, xy: (i32, i32)) -> Sprite<'a> {
        let (x, y) = xy;
        Sprite::new(
            &self.spritesheet, 
            Rect::new(
                x * self.tile_width as i32, 
                y * self.tile_height as i32, 
                self.tile_width, 
                self.tile_height
            )
        )
    }

    pub fn multisprite(&self, xy: (i32, i32), size: (u32, u32)) -> Sprite<'a> {
        let ((x, y), (width, height)) = (xy, size);
        Sprite::new(
            &self.spritesheet, 
            Rect::new(
                x * self.tile_width as i32, 
                y * self.tile_height as i32, 
                self.tile_width * width, 
                self.tile_height * height
            )
        )
    }

    pub fn tile(&self, x: i32, y: i32) -> Rect {
        Rect::new(x * self.tile_width as i32, y * self.tile_height as i32, self.tile_width, self.tile_height)
    }

    pub fn batch(&self) -> SpriteBatch<'a> {
        SpriteBatch::new(&self.spritesheet)
    }
}