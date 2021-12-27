use crate::game_loop::GameLoop;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::app::assets::Assets;
use crate::app::events::*;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Coin<'a> {
    pub x: f64,
    pub y: f64,
    pub id: u32,
    pub collected: bool,
    pub sprite: Sprite<'a>,
    mesh: ConvexMesh
}

impl <'a> Coin<'a> {  
    pub fn new(x: f64, y: f64, width: u32, height: u32, id: u32, assets: &'a Assets<'a>) -> Self {
        Coin {
            x,
            y,
            id,
            collected: false,
            sprite: assets.sprite(1, 0),
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, GEvent> for Coin<'a> {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.draw(&Layer::FOREGROUND, &self.sprite, self.x as i32, self.y as i32);
        Ok(())
    }

    fn event(&mut self, event: &Event, _events: &mut Events) -> Result<(), String> {
        match event {
            Event::Game(GEvent::CoinCollected(id)) if id == &self.id => { self.collected = true; },
            _ => { }    
        }

        Ok(())
    }
}