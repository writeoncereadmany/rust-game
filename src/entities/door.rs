use std::time::Duration;

use sdl2::event::Event;

use crate::game_loop::GameEvents;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::app::assets::Assets;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Door<'a> {
    pub x: f64,
    pub y: f64,
    pub sprite: Sprite<'a>,
    mesh: ConvexMesh
}

impl <'a> Door<'a> {  
    pub fn new(x: f64, y: f64, width: u32, height: u32, assets: &'a Assets<'a>) -> Self {
        Door {
            x,
            y,
            sprite: assets.sprite(1, 1),
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for Door<'a> {

    fn update(&mut self, _dt: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.draw(&Layer::FOREGROUND, &self.sprite, self.x as i32, self.y as i32);
        Ok(())
    }

    fn on_event(&mut self, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}