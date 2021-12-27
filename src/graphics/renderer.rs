use std::collections::BTreeMap;
use std::fmt::Debug;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{WindowContext};

use super::sprite::{ Sprite, SpriteSheet };

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Layer {
    BACKGROUND,
    FOREGROUND
}

pub enum Justification {
    LEFT,
    CENTER,
    RIGHT,

}

pub struct Renderer<'a, T> 
where T: Ord + Debug
{
    canvas: WindowCanvas,
    layers: BTreeMap<T, Texture<'a>>,
    spritesheet: SpriteSheet<'a>,
    spritefont: SpriteSheet<'a>,
    source_rect: Rect,
    target_rect: Rect,
    text_width: u32,
}

impl <'a, T> Renderer<'a, T>
where T: Ord + Debug
{
    pub fn new(
        canvas: WindowCanvas, 
        texture_creator: &'a TextureCreator<WindowContext>, 
        spritesheet: SpriteSheet<'a>, 
        spritefont: SpriteSheet<'a>,
        width: u32, 
        height: u32,
        text_width: u32, 
        layers: Vec<T>
    ) -> Result<Self, TextureValueError>
    {
        let source_rect = Rect::new(0, 0, width, height);
        let target_rect = calculate_target_rect(&canvas, width, height);
        let mut layer_map = BTreeMap::new();
        for layer in layers {
            let mut texture: Texture<'a> = texture_creator.create_texture_target(None, width, height)?;
            texture.set_blend_mode(BlendMode::Blend);
            layer_map.insert(layer, texture);
        }
        Ok(Renderer {
            canvas,
            layers: layer_map,
            spritesheet,
            spritefont,
            source_rect,
            target_rect,
            text_width,
        })
    }

    pub fn draw_tile(&mut self, layer: &T, tile: (i32, i32), x: i32, y: i32) {
        self.draw(layer, &self.spritesheet.sprite(tile), x, y);
    }

    pub fn draw_text(&mut self, text: String, layer: &T, x: i32, y: i32, justification: Justification) {
        let text_width = text.len() as i32 * self.text_width as i32;
        let mut current_x = match justification {
            Justification::LEFT => x,
            Justification::CENTER => x - (text_width / 2),
            Justification::RIGHT => x - text_width,
        };

        for ch in text.chars() {
            self.draw(layer, &self.spritefont.sprite(tile(ch)), current_x, y);
            current_x += self.text_width as i32;
        }
    }

    pub fn draw(&mut self, layer: &T, sprite: &Sprite<'a>, x: i32, y: i32) {
        let texture: &mut Texture<'a> = self.layers.get_mut(layer).unwrap();
        let corrected_y = (self.source_rect.height() as i32 - y) - sprite.source_rect.height() as i32;
        self.canvas.with_texture_canvas(texture, |c| { 
            c.copy(sprite.spritesheet, sprite.source_rect, Rect::new(x, corrected_y, sprite.source_rect.width(), sprite.source_rect.height())).unwrap();
        }).unwrap();
    }

    pub fn clear(&mut self, layer: &T) -> Result<(), TargetRenderError> {
        let texture: &mut Texture<'a> = self.layers.get_mut(layer).unwrap();
        self.canvas.with_texture_canvas(texture, |c| {
            c.set_draw_color(Color::from((0, 0, 0, 0)));
            c.clear();
            ()
        })
    }

    pub fn present(&mut self) -> Result<(), String>
    where
    {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        for (_, texture) in self.layers.iter_mut() {
            self.canvas.copy(&texture, None, self.target_rect)?;
        }
        self.canvas.present();
        Ok(())
    }
}

fn calculate_target_rect(canvas: &WindowCanvas, width: u32, height: u32) -> Rect {
    let (window_width, window_height) = canvas.window().size();
    let x_scale = window_width as f64 / width as f64;
    let y_scale = window_height as f64 / height as f64;
    let scale = f64::min(x_scale, y_scale);

    let scaled_width = (width as f64 * scale) as u32;
    let x_offset = (window_width - scaled_width) / 2;
    let scaled_height = (height as f64 * scale) as u32;
    let y_offset = (window_height - scaled_height) / 2;

    Rect::new(x_offset as i32, y_offset as i32, scaled_width, scaled_height)    
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