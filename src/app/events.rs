use component_derive::Event;

use crate::events::EventTrait;

#[derive(Event)]
pub struct CoinCollected;
#[derive(Event)]
pub struct BellCollected;

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct ReachedDoor;

#[derive(Event)]
pub struct Destroy(pub u64);
