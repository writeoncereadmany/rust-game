use component_derive::Constant;
use entity::{ entity, Entities, Id };
use engine::graphics::sprite::Sprite;
use engine::events::*;
use engine::shapes::shape::shape::Shape;
use crate::app::events::{KeyCollected, Destroy, SpawnParticle};
use super::components::*;

#[derive(Clone, Constant)]
pub struct Lockbox;

pub fn spawn_lockbox(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Lockbox)
        .with(Obstacle)
        .with(Position(x, y))
        .with(Sprite::new(4, 8, 0.5, "Sprites"))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
    );
}

pub fn lockbox_events(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|key| open_lockboxes(key, entities, events));
}

pub fn open_lockboxes(_key: &KeyCollected, entities: &mut Entities, events: &mut Events) {
    entities.apply(|(Lockbox, Position(x, y), Id(id))| {
        events.fire(Destroy(id));
        events.fire(SpawnParticle(x, y));
    });
}