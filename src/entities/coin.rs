use std::time::Duration;

use crate::game_loop::*;
use crate::graphics::renderer::Renderer;
use crate::app::events::CoinCollected;
use crate::shapes::convex_mesh::ConvexMesh;

const ROTATION_FPS : f64 = 5.0;

pub struct Coin {
    pub x: f64,
    pub y: f64,
    pub id: u32,
    pub collected: bool,
    pub existed_for: Duration,
    pub phase_offset: f64,
    mesh: ConvexMesh
}

impl Coin {  
    pub fn new(x: f64, y: f64, id: u32, phase_offset: f64) -> Self {
        Coin {
            x,
            y,
            id,
            collected: false,
            existed_for: Duration::ZERO,
            phase_offset,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for Coin {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        let phase = (self.existed_for.as_secs_f64() * ROTATION_FPS) + self.phase_offset;
        let frame = (phase.round() as i32).rem_euclid(4);
        renderer.draw_tile((frame, 3), self.x, self.y);
        Ok(())
    }

    fn event(&mut self, event: &Event, _events: &mut Events) -> Result<(), String> {
        event.apply(|CoinCollected(id)| {
            if id == &self.id { self.collected = true; }
        });
        event.apply(|dt| self.existed_for += *dt);
        Ok(())
    }
}