use entity::{ entity, Component, Entities };

use component_derive::Constant;

use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;

use super::components::*;

#[derive(Constant)]
pub struct Door;

pub fn spawn_door(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Door)
        .with(Position(x, y))
        .with(Sprite::new(1, 0, 0.5))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}