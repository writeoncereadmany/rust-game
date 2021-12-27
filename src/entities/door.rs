use crate::game_loop::GameLoop;
use crate::graphics::renderer::{Layer, Renderer};
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Door {
    pub x: f64,
    pub y: f64,
    mesh: ConvexMesh
}

impl Door {  
    pub fn new(x: f64, y: f64, width: u32, height: u32) -> Self {
        Door {
            x,
            y,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a, Layer>, f64> for Door {

    fn render(&self, renderer: &mut Renderer<'a, Layer>) -> Result<(), String> {
        renderer.draw_tile(&Layer::FOREGROUND, (1, 1), self.x as i32, self.y as i32);
        Ok(())
    }
}