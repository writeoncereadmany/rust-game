use component_derive::Event;

use crate::events::EventTrait;

#[derive(Event)]
pub struct CoinCollected(pub u32);

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct ReachedDoor;
