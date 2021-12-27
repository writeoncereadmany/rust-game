use super::sprite::Sprite;
use super::lo_res_renderer::LoResRenderer;
use super::renderer::Renderer;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::fmt::Debug;

pub struct SpriteFont<'a> {
    spritesheet: &'a Texture<'a>,
    char_width: u32,
    char_height: u32
}

pub enum Justification {
    LEFT,
    CENTER,
    RIGHT,

}

impl <'a> SpriteFont<'a> {

    pub fn new(spritesheet: &'a Texture<'a>, char_width: u32, char_height: u32) -> Self {
        SpriteFont { spritesheet, char_width, char_height }
    }

    pub fn render<Layer>(&self, text: String, x: i32, y: i32, renderer: &mut LoResRenderer<'a, Layer>, layer: &Layer, justification: Justification) 
    where Layer : Ord + Debug {
        let text_width = text.len() as i32 * self.char_width as i32;
        let mut current_x = match justification {
            Justification::LEFT => x,
            Justification::CENTER => x - (text_width / 2),
            Justification::RIGHT => x - text_width,
        };

        for ch in text.chars() {
            renderer.draw(layer, &self.char(ch), current_x, y);
            current_x += self.char_width as i32;
        }
    }

    fn char(&self, character: char) -> Sprite<'a> {
        match character {
            '0'..='9' => self.number(character), 
            'a'..='z' => self.lowercase(character),
            'A'..='Z' => self.uppercase(character),
            ':' => self.tile(6, 3),
            '-' => self.tile(7, 3),
            '?' => self.tile(8, 3),
            '!' => self.tile(9, 3),
            '.' => self.tile(6, 6),
            ',' => self.tile(7, 6),
            ' ' => self.tile(8, 6),
            _ => self.tile(9, 6),
        }
    }

    fn number(&self, num: char) -> Sprite<'a> {
        Sprite::new(self.spritesheet, self.rect(num, '0', 0))
    }

    fn lowercase(&self, letter: char) -> Sprite<'a> {
        Sprite::new(self.spritesheet, self.rect(letter, 'a', 1))
    }

    fn uppercase(&self, letter: char) -> Sprite<'a> {
        Sprite::new(self.spritesheet, self.rect(letter, 'A', 4))
    }

    fn tile(&self, x: i32, y: i32) -> Sprite<'a> {
        Sprite::new(self.spritesheet, Rect::new(x * self.char_width as i32, y * self.char_height as i32, self.char_width, self.char_height))
    }

    fn rect(&self, ch: char, base: char, starting_row: i32) -> Rect {
        let offset = SpriteFont::offset(ch, base);
        let x = offset % 10;
        let y = (offset / 10) + starting_row;
        Rect::new(x * self.char_width as i32, y * self.char_height as i32, self.char_width, self.char_height)
    }

    fn offset(ch: char, base: char) -> i32 {
        ch as i32 - base as i32
    }
}