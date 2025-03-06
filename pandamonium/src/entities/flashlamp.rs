use std::time::Duration;
use component_derive::{Event, Variable};
use engine::events::{Event, EventTrait, Events};
use crate::entities::components::Position;
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities, Id};

#[derive(Event)]
pub struct LightFlashbulb(pub u64);

#[derive(Variable, Clone)]
pub struct TimeToFire(pub f64);

pub fn spawn_flashlamp(x: f64, y: f64, fire_in: f64, entities: &mut Entities) -> u64 {
    entities.spawn(
        entity()
            .with(Position(x, y))
            .with(Sprite::new(6, 4, 0.0, "Walls"))
            .with(TimeToFire(fire_in)),
    )
}

pub fn flashbulb_events(entities: &mut Entities, event: &Event, events: &mut Events) {
    event.apply(|duration: &Duration| entities.apply(|(Id(id), TimeToFire(time_left))| {
        let new_time_left = time_left - duration.as_secs_f64();
        if new_time_left < 0.0 {
            events.fire(LightFlashbulb(id));
            None
        } else {
            Some(TimeToFire(new_time_left))
        }
    }));
    event.apply(|LightFlashbulb(entity_id)| entities.apply_to(entity_id, |()| Sprite::new(7, 4, 0.0, "Walls")));
}