use component_derive::Event;

use crate::game_loop::EventTrait;

#[derive(Event)]
pub struct CoinCollected(pub u32);

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct ReachedDoor;
