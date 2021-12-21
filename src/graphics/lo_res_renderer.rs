use std::collections::BTreeMap;
use std::fmt::Debug;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, WindowCanvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{WindowContext};

use super::renderer::Renderer;
use super::sprite::Sprite;

pub struct LoResRenderer<'a, T> 
where T: Ord + Debug
{
    canvas: WindowCanvas,
    layers: BTreeMap<T, Texture<'a>>,
    source_rect: Rect,
    target_rect: Rect,
}

impl <'a, T> LoResRenderer<'a, T>
where T: Ord + Debug
{
    // Creates a new LoResRenderer with the given width and height, for the given canvas.
    pub fn new(canvas: WindowCanvas, texture_creator: &'a TextureCreator<WindowContext>, width: u32, height: u32, layers: Vec<T>) 
    -> Result<Self, TextureValueError>
    {
        let source_rect = Rect::new(0, 0, width, height);
        let target_rect = calculate_target_rect(&canvas, width, height);
        let mut layer_map = BTreeMap::new();
        for layer in layers {
            let mut texture: Texture<'a> = texture_creator.create_texture_target(None, width, height)?;
            texture.set_blend_mode(BlendMode::Blend);
            layer_map.insert(layer, texture);
        }
        Ok(LoResRenderer {
            canvas,
            layers: layer_map,
            source_rect,
            target_rect,
        })
    }
}

impl <'a, Layer> Renderer<'a, Layer> for LoResRenderer<'a, Layer>
where Layer : Ord + Debug {
    fn draw(&mut self, layer: &Layer, sprite: &Sprite<'a>, x: i32, y: i32) {
        let texture: &mut Texture<'a> = self.layers.get_mut(layer).unwrap();
        let corrected_y = (self.source_rect.height() as i32 - y) - sprite.source_rect.height() as i32;
        self.canvas.with_texture_canvas(texture, |c| { 
            c.copy(sprite.spritesheet, sprite.source_rect, Rect::new(x, corrected_y, sprite.source_rect.width(), sprite.source_rect.height())).unwrap();
        }).unwrap();
    }

    fn clear(&mut self, layer: &Layer) -> Result<(), TargetRenderError> {
        let texture: &mut Texture<'a> = self.layers.get_mut(layer).unwrap();
        self.canvas.with_texture_canvas(texture, |c| {
            c.set_draw_color(Color::from((0, 0, 0, 0)));
            c.clear();
            ()
        })
    }

    fn present(&mut self) -> Result<(), String>
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