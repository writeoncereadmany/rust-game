use component_derive::Constant;
use entity::{ entity, Component, Entities };

use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;
use super::components::*;

#[derive(Constant)]
pub struct Bell;

pub fn spawn_bell(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Bell)
        .with(Position(x, y))
        .with(Sprite::new(1, 0, 0.5))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}