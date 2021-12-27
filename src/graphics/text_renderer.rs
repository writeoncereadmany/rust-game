use super::sprite::{ SpriteSheet };
use super::renderer::Renderer;
use sdl2::render::Texture;
use std::fmt::Debug;

pub struct SpriteFont<'a> {
    spritesheet: SpriteSheet<'a>,
    char_width: u32
}

pub enum Justification {
    LEFT,
    CENTER,
    RIGHT,

}

impl <'a> SpriteFont<'a> {

    pub fn new(spritesheet: &'a Texture<'a>, char_width: u32, char_height: u32) -> Self {
        let spritesheet = SpriteSheet::new(spritesheet, char_width, char_height);
        SpriteFont { spritesheet, char_width }
    }

    pub fn render<Layer>(&self, text: String, x: i32, y: i32, renderer: &mut Renderer<'a, Layer>, layer: &Layer, justification: Justification) 
    where Layer : Ord + Debug {
        let text_width = text.len() as i32 * self.char_width as i32;
        let mut current_x = match justification {
            Justification::LEFT => x,
            Justification::CENTER => x - (text_width / 2),
            Justification::RIGHT => x - text_width,
        };

        for ch in text.chars() {
            renderer.draw(layer, &self.spritesheet.sprite(tile(ch)), current_x, y);
            current_x += self.char_width as i32;
        }
    }
}

fn tile(ch: char) -> (i32, i32) {
    match ch {
        '0'..='9' => position(ch, '0', 0), 
        'a'..='z' => position(ch, 'a', 1),
        'A'..='Z' => position(ch, 'A', 4),
        ':' => (6, 3),
        '-' => (7, 3),
        '?' => (8, 3),
        '!' => (9, 3),
        '.' => (6, 6),
        ',' => (7, 6),
        ' ' => (8, 6),
        _ => (9, 6),
    }
}

fn position(ch: char, base: char, starting_row: i32) -> (i32, i32) {
    let offset = ch as i32 - base as i32;
    (offset % 10, (offset / 10) + starting_row)
}