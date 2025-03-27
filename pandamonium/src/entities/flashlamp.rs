use std::time::Duration;
use component_derive::{Event, Variable};
use engine::events::{Event, EventTrait, Events};
use crate::entities::components::Position;
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities, Id};
use FBColor::{OFF, RED};
use crate::entities::flashlamp::FBColor::YELLOW;

#[derive(Event)]
pub struct LightFlashbulb(pub u64);

#[derive(Event)]
pub struct TurnFlashbulbsYellow;

#[derive(Event)]
pub struct TurnFlashbulbsRed;

#[derive(Variable, Clone)]
pub struct TimeToFire(pub f64);

#[derive(Clone)]
enum FBColor {
    GREEN,
    YELLOW,
    RED,
    OFF
}

#[derive(Variable, Clone)]
pub struct FlashbulbColor(pub FBColor);

pub fn spawn_flashlamp(x: f64, y: f64, fire_in: f64, entities: &mut Entities) -> u64 {
    entities.spawn(
        entity()
            .with(Position(x, y))
            .with(flashbulb_sprite(OFF))
            .with(TimeToFire(fire_in))
            .with(FlashbulbColor(FBColor::GREEN)),
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

    event.apply(|TurnFlashbulbsYellow| entities.apply(|(FlashbulbColor(_), maybeTimeout)| {
        if let Some(TimeToFire(_)) = maybeTimeout {
            (flashbulb_sprite(OFF), FlashbulbColor(YELLOW))
        } else {
            (flashbulb_sprite(YELLOW), FlashbulbColor(YELLOW))
        }
    }));
    event.apply(|TurnFlashbulbsRed| entities.apply(|(FlashbulbColor(_), maybeTimeout)| {
        if let Some(TimeToFire(_)) = maybeTimeout {
            (flashbulb_sprite(OFF), FlashbulbColor(RED))
        } else {
            (flashbulb_sprite(RED), FlashbulbColor(RED))
        }
    }));

    event.apply(|LightFlashbulb(entity_id)| entities.apply_to(entity_id, |(FlashbulbColor(fb_color))|
        flashbulb_sprite(fb_color))
    );
}

fn flashbulb_sprite(fb_color: FBColor) -> Sprite {
    match fb_color {
        FBColor::GREEN => Sprite::new(7, 4, 3.0, "Walls"),
        YELLOW => Sprite::new(7, 5, 3.0, "Walls"),
        RED => Sprite::new(7, 6, 3.0, "Walls"),
        OFF => Sprite::new(6, 4, 3.0, "Walls")
    }
}