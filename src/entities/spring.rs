use entity::{ entity, Entities };

use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;

use super::components::*;

pub fn spawn_spring(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x, y))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(Sprite::new(0, 8, 0.7))
    );
}