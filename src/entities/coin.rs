use crate::game_loop::GameLoop;
use crate::graphics::renderer::Renderer;
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
    pub fn new(x: f64, y: f64, id: u32) -> Self {
        Coin {
            x,
            y,
            id,
            collected: false,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>, GEvent> for Coin {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_tile((1, 0), self.x, self.y);
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