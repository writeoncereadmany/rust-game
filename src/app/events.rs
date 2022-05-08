use component_derive::Event;

use crate::events::EventTrait;

#[derive(Event)]
pub struct CoinCollected {
    pub id: u64,
    pub x: f64,
    pub y: f64
}

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct ReachedDoor;

#[derive(Event)]
pub struct Destroy(pub u64);
