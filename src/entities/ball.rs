use sdl2::rect::Rect;

use crate::app::assets::Assets;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Ball<'a> {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub sprite: Sprite<'a>,
    pub last_push: (f64, f64),
    mesh: ConvexMesh
}

impl <'a> Ball<'a> {  
    pub fn new(x: f64, y: f64, width: u32, height: u32, assets: &'a Assets<'a>) -> Self {
        let sprite = Sprite::new(&assets.spritesheet, Rect::new(0, 0, width, height));
        Ball {
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            sprite,
            last_push: (0.0, 0.0),
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}