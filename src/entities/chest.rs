use std::time::Duration;
use component_derive::{ Constant, Event };
use entity::{ entity, Entities, not };

use entity::Id;
use crate::graphics::sprite::Sprite;
use crate::events::{ EventTrait, Events, Event };
use crate::shapes::convex_mesh::ConvexMesh;
use crate::app::events::*;
use super::components::*;
use super::pickup::*;

#[derive(Clone, Constant)]
pub struct Chest;

#[derive(Event)]
pub struct OpenChest { id: u64 }

pub fn spawn_chest(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Chest)
        .with(Position(x, y))
        .with(Sprite::new(2, 7, 0.5))
    );
}

pub fn spawn_open_chest(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let chest_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(3, 7, 0.5))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );

    events.schedule(Duration::from_secs(1), Destroy(chest_id));
}

pub fn spawn_ruby(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y + 0.1))
        .with(Velocity(0.0, 20.0))
        .with(Gravity)
        .with(Sprite::new(3, 8, 0.75))
        .with(OnPickupDo::Score(100))
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupText("100"))
        .with(ReferenceMesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![])))
    );
}

pub fn open_chests(_key: &KeyCollected, entities: &mut Entities, events: &mut Events) {
    entities.apply(|(Chest, Id(id))| {
        events.fire(OpenChest { id });
    });
}

pub fn open_chest(OpenChest { id } : &OpenChest, entities: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = entities.delete(id) {
        spawn_open_chest(x, y, entities, events);
        spawn_ruby(x, y, entities);
    };
}

pub fn chest_events(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|key| open_chests(key, entities, events));
    event.apply(|chest| open_chest(chest, entities, events));
}