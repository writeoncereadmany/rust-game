use component_derive::{Constant, Event};
use engine::audio::audio::*;
use engine::events::{EventTrait, Events};
use entity::Entities;

use crate::app::events::*;

use super::components::*;
use super::particle::{spawn_spangle, spawn_text};

#[derive(Clone, Constant)]
pub struct Pickup;

#[derive(Event)]
pub struct PickupCollected(pub u64);

#[derive(Clone, Constant)]
pub enum OnPickupEffect {
    Sparkles
}

#[derive(Clone, Constant)]
pub struct OnPickupTune(pub PlayTune);

#[derive(Clone, Constant)]
pub struct OnPickupText(pub &'static str);

#[derive(Clone, Constant)]
pub enum OnPickupDo {
    Score(u32),
    CollectFruit(u32),
    DoubleScore,
    OpenChests,
    CompleteLevel(String),
}

pub fn collect_pickup(PickupCollected(id): &PickupCollected, entities: &mut Entities, events: &mut Events)
{
    if let Some((Position(x, y), effect, tune, text, action)) = entities.delete(&id)
    {
        if let Some(OnPickupEffect::Sparkles) = effect { spawn_spangle(x, y, entities, events); }
        if let Some(OnPickupTune(tune)) = tune { events.fire(tune); }
        if let Some(OnPickupText(text)) = text { spawn_text(x + 0.5, y + 1.0, text, entities, events); }
        match action {
            Some(OnPickupDo::Score(score)) => events.fire(Score::Points(score)),
            Some(OnPickupDo::CollectFruit(score)) => events.fire(Score::Fruit(score)),
            Some(OnPickupDo::DoubleScore) => events.fire(Score::Double),
            Some(OnPickupDo::OpenChests) => events.fire(KeyCollected),
            Some(OnPickupDo::CompleteLevel(next_level)) => events.fire(ReachedDoor(next_level)),
            _ => {}
        }
    }
}