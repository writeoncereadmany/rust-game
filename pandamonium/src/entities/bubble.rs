use std::time::Duration;
use component_derive::{Constant, Event};
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};
use engine::events::*;
use engine::shapes::shape::shape::Shape;
use crate::app::events::Destroy;
use crate::entities::hero::JumpDirection::UP;
use crate::entities::hero::Jumped;
use super::components::*;

#[derive(Clone, Constant)]
pub struct Bubble;

#[derive(Event)]
pub struct BubbleHit(pub (f64, f64), pub u64);

pub fn spawn_bubble(x: f64, y: f64, entities: &mut Entities) {
    let phase = phase_offset(x, y);
    let animation_cycle = AnimationCycle(vec!(
        (0.25, Sprite::new(5, 3, 0.5, "Sprites")),
        (0.5, Sprite::new(6, 3, 0.5, "Sprites")),
        (0.75, Sprite::new(5, 3, 0.5, "Sprites")),
        (1.0, Sprite::new(7, 3, 0.5, "Sprites"))));
    entities.spawn(entity()
        .with(Bubble)
        .with(Position(x, y))
        .with(next_frame(phase, &animation_cycle))
        .with(Period(1.5))
        .with(Phase(phase_offset(x, y)))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
        .with(animation_cycle)
    );
}

pub fn bubble_hit(BubbleHit((_px, py), id): &BubbleHit, events: &mut Events, entities: &mut Entities) {
    if let Some(Position(x, y)) = entities.delete(&id) {
        let pop_id = entities.spawn(entity()
            .with(AnimationCycle(vec!(
                (0.33, Sprite::new(4, 1, 0.5, "Sprites")),
                (0.66, Sprite::new(5, 1, 0.5, "Sprites")),
                (1.00, Sprite::new(6, 1, 0.5, "Sprites")),
            )))
            .with(Sprite::new(4, 1, 0.5, "Sprites"))
            .with(Phase(0.0))
            .with(Period(0.3))
            .with(Position(x, y))
        );
        events.schedule("world", Duration::from_secs_f64(0.3), Destroy(pop_id))
    }
    if *py > 0.0 {
        events.fire(Jumped(UP))
    }
}

fn phase_offset(x: f64, y: f64) -> f64 {
    // magic numbers which don't mean anything, but feel good
    x * 0.8 + y * 0.4
}