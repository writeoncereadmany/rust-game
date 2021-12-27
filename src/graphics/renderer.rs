use std::collections::BTreeMap;
use std::fmt::Debug;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{WindowContext};

use super::sprite::{ Sprite, SpriteSheet };
use super::text_renderer::{ Justification, SpriteFont };

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum Layer {
    BACKGROUND,
    FOREGROUND
}

pub struct Renderer<'a, T> 
where T: Ord + Debug
{
    canvas: WindowCanvas,
    layers: BTreeMap<T, Texture<'a>>,
    spritesheet: SpriteSheet<'a>,
    spritefont: SpriteFont<'a>,
    source_rect: Rect,
    target_rect: Rect,
}

impl <'a, T> Renderer<'a, T>
where T: Ord + Debug
{
    // Creates a new LoResRenderer with the given width and height, for the given canvas.
    pub fn new(
        canvas: WindowCanvas, 
        texture_creator: &'a TextureCreator<WindowContext>, 
        spritesheet: SpriteSheet<'a>, 
        spritefont: SpriteFont<'a>,
        width: u32, 
        height: u32, 
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
        })
    }

    pub fn draw_tile(&mut self, layer: &T, tile: (i32, i32), x: i32, y: i32) {
        self.draw(layer, &self.spritesheet.sprite(tile), x, y);
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