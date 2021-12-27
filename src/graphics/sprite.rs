use sdl2::rect::Rect;
use sdl2::render::Texture;

#[derive(Clone)]
pub struct Sprite<'a> {
    pub spritesheet: &'a Texture<'a>,
    pub source_rect: Rect
}

pub trait Sprited<'a> {
    fn sprite(&self) -> &Sprite<'a>; 
}

impl <'a> Sprite<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, source_rect: Rect) -> Self {
        Sprite{
            spritesheet,
            source_rect
        }
    }
}