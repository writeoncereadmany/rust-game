use component_derive::Constant;
use entity::{ entity, Component, Entities };
use crate::graphics::sprite::Sprite;

use crate::shapes::convex_mesh::ConvexMesh;

use super::components::*;

const ANIMATION_CYCLE: [(f64, Sprite);4] = [
    (0.25, Sprite::new(0,3)), 
    (0.5, Sprite::new(1, 3)), 
    (0.75, Sprite::new(2, 3)), 
    (1.0, Sprite::new(3,3))];

#[derive(Constant)]
pub struct Coin;

pub fn spawn_coin(x: f64, y: f64, entities: &mut Entities) {
    let phase = phase_offset(x, y);
    entities.spawn(entity()
        .with(Coin)
        .with(FixedPosition(x, y))
        .with(next_frame(&phase, &ANIMATION_CYCLE.to_vec()))
        .with(Period(0.7))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(Phase(phase_offset(x, y)))
        .with(AnimationCycle(ANIMATION_CYCLE.to_vec()))
    );
}

fn phase_offset(x: f64, y: f64) -> f64 {
    // magic numbers which don't mean anything, but feel good
    x * 0.8 + y * 0.4
}