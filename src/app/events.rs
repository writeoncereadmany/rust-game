use component_derive::Event;

use crate::{events::EventTrait, entities::hero::PandaType};

#[derive(Event)]
pub struct FlagpoleCollected { pub id: u64 }

#[derive(Event)]
pub struct KeyCollected;

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct ReachedDoor;

#[derive(Event)]
pub struct Destroy(pub u64);

#[derive(Event)]
pub struct SpawnParticle(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnText(pub f64, pub f64, pub String);

#[derive(Event)]
pub struct SpawnBulb(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnFlashBulb(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnHero(pub f64, pub f64, pub PandaType);

#[derive(Event)]
pub struct SpawnTimer(pub f64, pub f64);


#[derive(Clone, Event)]
pub enum Score {
    Points(u32),
    Double
}
