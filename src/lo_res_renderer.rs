use std::collections::BTreeMap;
use std::fmt::Debug;

use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{Window, WindowContext};

pub struct LoResRenderer<'a, T> 
where T: Ord + Debug
{
    canvas: Canvas<Window>,
    layers: BTreeMap<T, Texture<'a>>,
    target_rect: Rect,
}

impl <'a, T> LoResRenderer<'a, T>
where T: Ord + Debug
{
    // Creates a new LoResRenderer with the given width and height, for the given canvas.
    pub fn new(canvas: Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, width: u32, height: u32, layers: Vec<T>) 
    -> Result<Self, TextureValueError>
    {
        let target_rect = calculate_target_rect(&canvas, width, height);
        let mut layer_map = BTreeMap::new();
        for layer in layers {
            let mut texture: Texture<'a> = texture_creator.create_texture_target(None, width, height)?;
            texture.set_blend_mode(BlendMode::Blend);
            layer_map.insert(layer, texture);
        }
        Ok(LoResRenderer {
            canvas,
            target_rect,
            layers: layer_map,
        })
    }

    pub fn draw_to<F>(&mut self, layer: &T, f: F) -> Result<(), TargetRenderError>
    where F: FnOnce(&mut Canvas<Window>) 
    {
        let texture: &mut Texture<'a> = self.layers.get_mut(layer).unwrap();
        self.canvas.with_texture_canvas(texture, f)
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

fn calculate_target_rect(canvas: &Canvas<Window>, width: u32, height: u32) -> Rect {
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