use crate::game_loop::GameLoop;
use crate::graphics::renderer::{Layer, Renderer};
use crate::app::events::*;
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Coin {
    pub x: f64,
    pub y: f64,
    pub id: u32,
    pub collected: bool,
    mesh: ConvexMesh
}

impl Coin {  
    pub fn new(x: f64, y: f64, width: u32, height: u32, id: u32) -> Self {
        Coin {
            x,
            y,
            id,
            collected: false,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a, Layer>, GEvent> for Coin {

    fn render(&self, renderer: &mut Renderer<'a, Layer>) -> Result<(), String> {
        renderer.draw_tile(&Layer::FOREGROUND, (1, 0), self.x as i32, self.y as i32);
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