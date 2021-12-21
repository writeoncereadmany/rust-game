use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;

pub struct Ball<'a> {
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub sprite: Sprite<'a>,
    mesh: ConvexMesh
}

impl <'a> Ball<'a> {  
    pub fn new(x: f64, y: f64, width: f64, height: f64, sprite: Sprite<'a>) -> Self {
        Ball {
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            sprite,
            mesh: ConvexMesh::new(vec![(0.0, 0.0), (width, 0.0), (width, height), (0.0, height)], vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}