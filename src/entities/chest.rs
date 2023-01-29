use std::time::Duration;
use component_derive::{ Constant, Event };
use entity::{ entity, Entities, not };

use entity::Id;
use crate::graphics::sprite::Sprite;
use crate::events::{ EventTrait, Events, Event };
use crate::shapes::convex_mesh::ConvexMesh;
use crate::app::events::*;
use super::components::*;
use super::particle::{spawn_spangle, spawn_text};

#[derive(Clone, Constant)]
pub struct Chest;


#[derive(Clone, Constant)]
pub struct Ruby;


#[derive(Clone, Constant)]
pub struct Floor(f64);

#[derive(Event)]
pub struct OpenChest { id: u64 }

pub fn spawn_chest(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Chest)
        .with(Position(x, y))
        .with(Sprite::new(2, 7, 0.5))
    );
}

pub fn open_chests(entities: &mut Entities, events: &mut Events) {
    entities.apply(|(Chest, Id(id))| {
        events.fire(OpenChest { id });
    });
}

pub fn open_chest(&OpenChest { id }: &OpenChest, entities: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = entities.delete(&id)
    {
        let chest_id = entities.spawn(entity()
            .with(Position(x, y))
            .with(Sprite::new(3, 7, 0.5))
            .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        );

        entities.spawn(entity()
            .with(Ruby)
            .with(Position(x, y + 0.1))
            .with(Velocity(0.0, 20.0))
            .with(Gravity)
            .with(Floor(y))
            .with(Sprite::new(3, 8, 0.75))
            .with(ReferenceMesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![])))
        );

        events.schedule(Duration::from_secs(1), Destroy(chest_id));
    }
}

#[derive(Event)]
struct ResetRuby(u64);

pub fn update_ruby(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|_dt : &Duration| entities.apply(|(Position(_, y), Floor(fy), Id(id))| {
        if y < fy {
            events.fire(ResetRuby(id))
        }
    }));
    event.apply(|ResetRuby(id)| entities.apply_to(id, |(Position(x, _), Floor(fy))| (Position(x, fy), not::<Gravity>(), not::<Velocity>(), not::<Translation>())));
}


pub fn collect_ruby(&RubyCollected { id }: &RubyCollected, entities: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = entities.delete(&id)
    {
        spawn_spangle(x, y, entities, events);
        spawn_text(x + 0.5, y + 1.0, "100", entities, events);    
    }
}