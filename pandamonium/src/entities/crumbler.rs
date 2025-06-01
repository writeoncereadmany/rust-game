use std::time::Duration;
use super::components::*;
use crate::app::events::{Destroy, KeyCollected, SpawnParticle};
use component_derive::{Constant, Event};
use engine::events::*;
use engine::graphics::sprite::Sprite;
use engine::shapes::shape::shape::Shape;
use entity::{entity, Entities, Id};

#[derive(Clone, Constant)]
pub struct Crumbler;

#[derive(Event)]
pub struct SpawnCrumbles(f64, f64);

pub fn spawn_crumbler(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Crumbler)
        .with(Obstacle)
        .with(Position(x, y))
        .with(Sprite::new(2, 0, 0.5, "Sprites"))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
    );
}

pub fn crumbler_events(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|collision| crumble(collision, entities, events));
    event.apply(|crumbles| spawn_crumbles(crumbles, events, entities));
}

pub fn crumble(SceneryCollision{ scenery_id, ..  }: &SceneryCollision, entities: &mut Entities, events: &mut Events) {
    entities.apply_to(scenery_id, |(Crumbler, Position(x, y))| {
        events.fire(Destroy(*scenery_id));
        events.fire(SpawnCrumbles(x, y));
    });
}

pub fn spawn_crumbles(SpawnCrumbles(x, y): &SpawnCrumbles, events: &mut Events, entities: &mut Entities) {
    println!("Crumbles spawned");
    let crumbler_id = entities.spawn(entity()
        .with(Obstacle)
        .with(AnimationCycle(vec!(
            (0.33, Sprite::new(3, 0, 0.5, "Sprites")),
            (0.66, Sprite::new(4, 0, 0.5, "Sprites")),
            (1.00, Sprite::new(5, 0, 0.5, "Sprites"))
        )))
        .with(Sprite::new(3, 0, 0.5, "Sprites"))
        .with(Phase(0.0))
        .with(Period(1.0))
        .with(Position(*x, *y))
        .with(TranslatedMesh(Shape::bbox(0.01, 0.01, 0.98, 0.98).translate(&(*x, *y))))
    );
    events.schedule("world", Duration::from_secs_f64(1.0), Destroy(crumbler_id));
}