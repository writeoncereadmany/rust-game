use sdl2::render::{ TargetRenderError };

use super::sprite::Sprite;


pub trait Renderer<'a, Layer> {
    fn draw(&mut self, layer: &Layer, sprite: &Sprite<'a>, x: i32, y: i32);

    fn clear(&mut self, layer: &Layer) -> Result<(), TargetRenderError>;
    
    fn present(&mut self) -> Result<(), String>;
}