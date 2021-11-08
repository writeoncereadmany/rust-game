use sdl2::rect::Rect;
use sdl2::render::{WindowCanvas, Texture};

pub struct Sprite<'a> {
    spritesheet: &'a Texture<'a>,
    source_rect: Rect
}

impl <'a> Sprite<'a> {
    pub fn new(spritesheet: &'a Texture<'a>, source_rect: Rect) -> Self {
        Sprite{
            spritesheet,
            source_rect
        }
    }

    pub fn draw_to(&self, canvas: &mut WindowCanvas, x: i32, y: i32) -> Result<(), String> {
        canvas.copy(self.spritesheet, self.source_rect, Rect::new(x, y, self.source_rect.width(), self.source_rect.height()))
    }
}

