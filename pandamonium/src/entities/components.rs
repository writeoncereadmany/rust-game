use std::time::Duration;

use component_derive::{Constant, Event, Variable};
use engine::graphics::sprite::Sprite;
use engine::shapes::shape::shape::Shape;
use engine::events::EventTrait;
use entity::*;

#[derive(Event)]
pub struct SceneryCollision {
    pub movable_id: u64,
    pub scenery_id: u64,
    pub push: (f64, f64)
}

#[derive(Clone, Variable)]
pub struct Gravity;

#[derive(Clone, Variable)]
pub struct Period(pub f64);

#[derive(Clone, Variable)]
pub struct Phase(pub f64);

pub fn phase(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(Period(period), Phase(phase))| Phase((phase + (dt.as_secs_f64() / period)) % 1.0));
}

// note: flicker only works as long as there's another controller setting the sprite! eg an animation cycle
#[derive(Clone, Variable)]
pub struct Flicker(pub bool);

pub fn flicker(entities: &mut Entities) {
    entities.apply(|(Flicker(flicker), sprite): (Flicker, Sprite)| if flicker { Some(sprite) } else { None });
    entities.apply(|Flicker(flicker)| Flicker(!flicker));
}

#[derive(Clone, Constant)]
pub struct AnimationCycle(pub Vec<(f64, Sprite)>);

pub fn animation_cycle(entities: &mut Entities) {
    entities.apply(|(Phase(phase), cycle)| next_frame(phase, &cycle));
}

#[derive(Clone, Constant)]
pub struct ReferenceMesh(pub Shape);

#[derive(Clone, Variable)]
pub struct TranslatedMesh(pub Shape);

#[derive(Clone, Constant)]
pub struct ReferenceContextMesh(pub Shape);

#[derive(Clone, Variable)]
pub struct TranslatedContextMesh(pub Shape);

#[derive(Clone, Constant)]
pub struct Obstacle;

#[derive(Clone, Copy, Constant, PartialEq, Eq)]
pub enum Interacts {
    Spring
}

#[derive(Clone, Constant)]
pub struct Collidable;

pub fn next_frame(phase: f64, AnimationCycle(frames): &AnimationCycle) -> Sprite {
    let phase = phase % 1.0;
    for (frame_limit, sprite) in frames {
        if &phase < frame_limit {
            return sprite.clone();
        }
    }
    Sprite::new(0, 0, 0.0, "Sprites")
}

#[derive(Debug, Clone, Variable)]
pub struct Position(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Velocity(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Acceleration(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Translation(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Age(pub f64);

pub fn age(dt: &Duration, entities: &mut Entities) {
    entities.apply(|Age(age)| Age(age + dt.as_secs_f64()));
}