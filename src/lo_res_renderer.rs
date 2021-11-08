use sdl2::pixels::{Color};
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Canvas, TargetRenderError, Texture, TextureCreator, TextureValueError};
use sdl2::video::{Window, WindowContext};

pub struct LoResRenderer<'a> {
    canvas: Canvas<Window>,
    target_rect: Rect,
    background: Texture<'a>,
    sprite_buffer: Texture<'a>
}

impl <'a> LoResRenderer<'a> {

    // Creates a new LoResRenderer with the given width and height, for the given canvas.
    pub fn new(canvas: Canvas<Window>, texture_creator: &'a TextureCreator<WindowContext>, width: u32, height: u32) 
    -> Result<Self, TextureValueError>
    {
        let target_rect = calculate_target_rect(&canvas, width, height);
        let background = texture_creator.create_texture_target(None, width, height)?;
        let mut sprite_buffer = texture_creator.create_texture_target(None, width, height)?;
        sprite_buffer.set_blend_mode(BlendMode::Blend);
        Ok(LoResRenderer {
            canvas,
            target_rect,
            background,
            sprite_buffer
        })
    }

    pub fn draw_to_background<F>(&mut self, f: F) -> Result<(), TargetRenderError>
    where 
    F: FnOnce(&mut Canvas<Window>) 
    {
        self.canvas.with_texture_canvas(&mut self.background, f)
    }

    pub fn draw<F>(&mut self, f: F) -> Result<(), TargetRenderError>
    where
    F: FnOnce(&mut Canvas<Window>)
    {
        self.canvas.with_texture_canvas(&mut self.sprite_buffer, |c| {
            c.set_draw_color(Color::from((0, 0, 0, 0)));
            c.clear();
            ()
        })?;
        self.canvas.with_texture_canvas(&mut self.sprite_buffer, f)
    }

    pub fn present(&mut self) -> Result<(), String>
    where
    {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.copy(&mut self.background, None, self.target_rect)?;
        self.canvas.copy(&mut self.sprite_buffer, None, self.target_rect)?;
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