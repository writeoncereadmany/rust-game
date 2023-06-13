use std::cmp::Ordering;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{WindowContext};

use component_derive::{ Variable };

use crate::fps_counter::FpsCounter;
use super::sprite::{ Sprite, SpriteBatch, SpriteSheet };

pub mod align {
    pub const CENTER: u8 = 0b_0000_0001;
    pub const LEFT: u8 = 0b_0000_0010;
    pub const RIGHT: u8 = 0b_0000_0100;
    pub const MIDDLE: u8 = 0b_0000_1000;
    pub const BOTTOM: u8 = 0b_0001_0000;
    pub const TOP: u8 = 0b_0010_0000;
}

#[derive(Clone, Variable)]
pub struct Text {
    pub text: String,
    pub justification: u8
}

pub struct Renderer<'a> 
{
    canvas: WindowCanvas,
    surface: Texture<'a>,
    spritesheet: SpriteSheet<'a>,
    spritefont: SpriteSheet<'a>,
    batch: SpriteBatch<'a>,
    textbatch: SpriteBatch<'a>,
    source_rect: Rect,
    target_rect: Rect,
    text_width: f64,
    text_height: f64,
    fps_counter: FpsCounter
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
        rows: u32
    ) -> Result<Self, TextureValueError>
    {
        let width = columns * spritesheet.tile_width;
        let height = rows * spritesheet.tile_height;
        let source_rect = Rect::new(0, 0, width, height);
        let target_rect = calculate_target_rect(&canvas, width, height);
        let mut surface: Texture<'a> = texture_creator.create_texture_target(None, width, height)?;
        let batch = spritesheet.batch();
        let textbatch = spritefont.batch();
        let text_width = spritefont.tile_width as f64 / spritesheet.tile_width as f64;
        let text_height = spritefont.tile_height as f64 / spritesheet.tile_height as f64;

        surface.set_blend_mode(BlendMode::Blend);
        Ok(Renderer {
            canvas,
            surface,
            spritesheet,
            spritefont,
            source_rect,
            target_rect,
            batch,
            textbatch,
            text_width,
            text_height,
            fps_counter: FpsCounter::new(30)
        })
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite, x: f64, y: f64) {
        self.batch.blit(
            *sprite, 
            x * self.spritesheet.tile_width as f64, 
            y * self.spritesheet.tile_height as f64,
        );
    }

    pub fn draw_text(&mut self, Text { text, justification }: &Text, x: f64, y: f64) {
        let text_width = text.len() as f64 * self.text_width as f64;
        let mut current_x = match (justification & align::LEFT > 0, justification & align::RIGHT > 0) {
            (true, false) => x,
            (false, true) => x - text_width,
            _ => x - (text_width / 2.0),
        };

        let y = match (justification & align::BOTTOM > 0, justification & align::TOP > 0) {
            (true, false) => y,
            (false, true) => y - self.text_height,
            _ => y - (self.text_height / 2.0),
        };

        for ch in text.chars() {
            let (tx, ty) = char_tile(ch);
            self.textbatch.blit(
                Sprite::new(tx, ty, 2.0), 
                current_x * self.spritesheet.tile_width as f64, 
                y * self.spritesheet.tile_height as f64,
            );
            current_x += self.text_width as f64;
        }
    }

    fn draw_batch(&mut self, mut batch: SpriteBatch<'a>) {
        let height = self.source_rect.height() as i32;
        batch.blits.sort_by(|(sprite1, _), (sprite2, _)| compare(&sprite1.z, &sprite2.z));
        self.canvas.with_texture_canvas(&mut self.surface, |c| { 
            for &(sprite, (x, y)) in &batch.blits {
                let Sprite { flip_x, flip_y, .. } = sprite;
                let source_rect = batch.source_rect(sprite);
                let corrected_y = (height - y) - source_rect.height() as i32;
                if (false, false) == (flip_x, flip_y) {
                    c.copy(
                        batch.spritesheet, 
                        source_rect, 
                        Rect::new(x, corrected_y, source_rect.width(), source_rect.height()),
                    ).unwrap();
                } else {
                    c.copy_ex(
                        batch.spritesheet, 
                        source_rect, 
                        Rect::new(x, corrected_y, source_rect.width(), source_rect.height()),
                        0.0,
                        None,
                        flip_x, 
                        flip_y
                    ).unwrap();
                }
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
        let batch = std::mem::replace(&mut self.batch, self.spritesheet.batch());
        self.draw_batch(batch);

        let textbatch = std::mem::replace(&mut self.textbatch, self.spritefont.batch());
        self.draw_batch(textbatch);

        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.copy(&mut self.surface, None, self.target_rect)?;
        self.canvas.present();
        self.fps_counter.on_frame();
        Ok(())
    }
}

fn compare(a: &f64, b: &f64) -> Ordering {
    match a.partial_cmp(b) {
        Some(ord) => ord,
        None => Ordering::Equal
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

fn char_tile(ch: char) -> (i32, i32) {
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