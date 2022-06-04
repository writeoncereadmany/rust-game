use component_derive::{Constant, Variable};
use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;
use entity::*;

#[derive(Variable)]
pub struct Age(pub f64);

#[derive(Constant)]
pub struct Period(pub f64);

#[derive(Variable)]
pub struct Phase(pub f64);

#[derive(Constant)]
pub struct AnimationCycle(pub Vec<(f64, Sprite)>);

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

#[derive(Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Constant)]
pub struct FixedPosition(pub f64, pub f64);

#[derive(Constant)]
pub struct Door;