use std::time::Duration;

use component_derive::{Constant, Variable};
use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;
use entity::*;

#[derive(Variable)]
pub struct Age(pub f64);

pub fn age(entities: &mut Entities, dt: &Duration) {
    entities.apply(|Age(age)| Age(age + dt.as_secs_f64()));
}

#[derive(Constant)]
pub struct Period(pub f64);

#[derive(Variable)]
pub struct Phase(pub f64);

pub fn phase(entities: &mut Entities, dt: &Duration) {
    entities.apply_2(|Period(period), Phase(phase)| Phase((phase + (dt.as_secs_f64() / period)) % 1.0));
} 

#[derive(Constant)]
pub struct AnimationCycle(pub Vec<(f64, Sprite)>);

pub fn animation_cycle(entities: &mut Entities) {
    entities.apply_2(|Phase(phase), cycle| next_frame(phase, cycle));
}

#[derive(Constant)]
pub struct ReferenceMesh(pub ConvexMesh);

#[derive(Variable)]
pub struct Mesh(pub ConvexMesh);

pub fn next_frame(phase: &f64, AnimationCycle(frames): &AnimationCycle) -> Sprite {
    let phase = phase % 1.0;
    for (frame_limit, sprite) in frames {
        if &phase < frame_limit {
            return *sprite
        }
    }
    Sprite::new(0, 0)
}

#[derive(Debug, Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Variable)]
pub struct Translation(pub f64, pub f64);