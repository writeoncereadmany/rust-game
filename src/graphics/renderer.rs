use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{WindowContext};

use crate::map::Map;
use super::sprite::{ Sprite, SpriteBatch, SpriteSheet };

pub enum Justification {
    LEFT,
    CENTER,
    RIGHT,

}

pub struct Renderer<'a> 
{
    canvas: WindowCanvas,
    surface: Texture<'a>,
    spritesheet: SpriteSheet<'a>,
    spritefont: SpriteSheet<'a>,
    source_rect: Rect,
    target_rect: Rect,
    text_width: u32,
}

pub trait Tiled {
    fn tile(&self) -> (i32, i32);
}

impl <'a> Renderer<'a>
{
    pub fn new(
        canvas: WindowCanvas, 
        texture_creator: &'a TextureCreator<WindowContext>, 
        spritesheet: SpriteSheet<'a>, 
        spritefont: SpriteSheet<'a>,
        columns: u32, 
        rows: u32,
        text_width: u32
    ) -> Result<Self, TextureValueError>
    {
        let width = columns * spritesheet.tile_width;
        let height = rows * spritesheet.tile_height;
        let source_rect = Rect::new(0, 0, width, height);
        let target_rect = calculate_target_rect(&canvas, width, height);
        let mut surface: Texture<'a> = texture_creator.create_texture_target(None, width, height)?;
        surface.set_blend_mode(BlendMode::Blend);
        Ok(Renderer {
            canvas,
            surface,
            spritesheet,
            spritefont,
            source_rect,
            target_rect,
            text_width,
        })
    }

    pub fn draw_tile(&mut self, tile: (i32, i32), x: f64, y: f64) {
        self.draw(&self.spritesheet.sprite(tile), x, y);
    }

    pub fn draw_multitile(&mut self, tile: (i32, i32), size: (u32, u32), x: f64, y: f64) {
        self.draw(&self.spritesheet.multisprite(tile, size), x, y);
    }

    pub fn draw_text(&mut self, text: String, x: f64, y: f64, justification: Justification) {
        let text_width = text.len() as f64 * self.text_width as f64;
        let mut current_x = match justification {
            Justification::LEFT => x,
            Justification::CENTER => x - (text_width / 2.0),
            Justification::RIGHT => x - text_width,
        };

        for ch in text.chars() {
            self.draw(&self.spritefont.sprite(tile(ch)), current_x, y);
            current_x += self.text_width as f64;
        }
    }

    pub fn draw_map<Tile>(&mut self, map: &Map<Tile>) 
    where Tile : Clone + Tiled,
    {
        let mut batch = self.spritesheet.batch();
        for (pos, t) in map {
            let (x, y) = t.tile();
            let source_rect = self.spritesheet.tile(x, y);
            batch.blit(source_rect, pos.min_x as f64, pos.min_y as f64);
        }
        self.draw_batch(batch);
    }

    fn draw(&mut self, sprite: &Sprite<'a>, x: f64, y: f64) {
        let x = x.round() as i32;
        let y = y.round() as i32;
        let corrected_y = (self.source_rect.height() as i32 - y) - sprite.source_rect.height() as i32;
        self.canvas.with_texture_canvas(&mut self.surface, |c| { 
            c.copy(sprite.spritesheet, sprite.source_rect, Rect::new(x, corrected_y, sprite.source_rect.width(), sprite.source_rect.height())).unwrap();
        }).unwrap();
    }

    fn draw_batch(&mut self, batch: SpriteBatch<'a>) {
        let height = self.source_rect.height() as i32;
        self.canvas.with_texture_canvas(&mut self.surface, |c| { 
            for (source_rect, (x, y)) in batch.blits {
                let corrected_y = (height - y) - source_rect.height() as i32;
                c.copy(batch.spritesheet, source_rect, Rect::new(x, corrected_y, source_rect.width(), source_rect.height())).unwrap();
            }
        }).unwrap();
    }

    pub fn clear(&mut self) -> Result<(), TargetRenderError> {
        self.canvas.with_texture_canvas(&mut self.surface, |c| {
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
        self.canvas.copy(&mut self.surface, None, self.target_rect)?;
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