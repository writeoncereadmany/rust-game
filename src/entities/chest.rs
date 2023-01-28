use component_derive::{ Constant, Event };
use entity::{ entity, Component, Entities };

use entity::Id;
use crate::graphics::sprite::Sprite;
use crate::events::{ EventTrait, Events };
use crate::shapes::convex_mesh::ConvexMesh;
use crate::app::events::*;
use super::components::*;
use super::particle::{spawn_spangle, spawn_text};

#[derive(Clone, Constant)]
pub struct Chest;

#[derive(Event)]
pub struct OpenChest { x: f64, y: f64, id: u64 }

pub fn spawn_chest(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Chest)
        .with(Position(x, y))
        .with(Sprite::new(2, 7, 0.5))
    );
}

pub fn open_chests(entities: &mut Entities, events: &mut Events) {
    entities.apply(|(Chest, Position(x, y), Id(id))| {
        events.fire(OpenChest { x, y, id });
    });
}

pub fn open_chest(&OpenChest { x, y, id }: &OpenChest, entities: &mut Entities) {
    if let Some(Position(x, y)) = entities.delete(&id)
    {
        entities.spawn(entity()
            .with(Chest)
            .with(Position(x, y))
            .with(Sprite::new(3, 7, 0.5))
            .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        );
    }
}

pub fn collect_chest(&ChestCollected { id }: &ChestCollected, entities: &mut Entities, events: &mut Events) {
    if let Some(Position(x, y)) = entities.delete(&id)
    {
        spawn_spangle(x, y, entities, events);
        spawn_text(x + 0.5, y + 0.5, "100", entities, events);    
    }
}