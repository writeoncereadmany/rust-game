use crate::entities::components::Position;
use crate::entities::flashlamp::FBColor::YELLOW;
use component_derive::{Event, Variable};
use engine::events::{Event, EventTrait, Events};
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};
use std::time::Duration;
use FBColor::{GREEN, OFF, RED};

#[derive(Event)]
pub struct LightFlashbulb(pub u64);

#[derive(Event)]
pub struct TurnFlashbulbsYellow;

#[derive(Event)]
pub struct TurnFlashbulbsRed;

#[derive(Clone, Variable)]
pub struct FlashbulbLit(bool);

#[derive(Clone)]
enum FBColor {
    GREEN,
    YELLOW,
    RED,
    OFF
}

#[derive(Variable, Clone)]
pub struct FlashbulbColor(pub FBColor);

pub fn spawn_flashlamp(x: f64, y: f64, fire_in: f64, entities: &mut Entities, events: &mut Events) -> u64 {
    let entity_id = entities.spawn(
        entity()
            .with(Position(x, y))
            .with(flashbulb_sprite(OFF))
            .with(FlashbulbLit(false))
            .with(FlashbulbColor(GREEN)),
    );

    events.schedule("world", Duration::from_secs_f64(fire_in), LightFlashbulb(entity_id));

    entity_id
}

pub fn flashbulb_events(entities: &mut Entities, event: &Event) {
    event.apply(|TurnFlashbulbsYellow| entities.apply(|FlashbulbColor(_)| FlashbulbColor(YELLOW)));

    event.apply(|TurnFlashbulbsYellow| entities.apply(|(FlashbulbLit(lit), sprite)|
        if lit { flashbulb_sprite(YELLOW) } else { sprite }
    ));

    event.apply(|TurnFlashbulbsRed| entities.apply(|FlashbulbColor(_)| FlashbulbColor(RED)));

    event.apply(|TurnFlashbulbsRed| entities.apply(|(FlashbulbLit(lit), sprite)|
        if lit { flashbulb_sprite(RED) } else { sprite }
    ));

    event.apply(|LightFlashbulb(entity_id)| entities.apply_to(entity_id, |(FlashbulbColor(fb_color))|
        (flashbulb_sprite(fb_color), FlashbulbLit(true))
    ));
}

fn flashbulb_sprite(fb_color: FBColor) -> Sprite {
    match fb_color {
        GREEN => Sprite::new(7, 4, 3.0, "Walls"),
        YELLOW => Sprite::new(7, 5, 3.0, "Walls"),
        RED => Sprite::new(7, 6, 3.0, "Walls"),
        OFF => Sprite::new(6, 4, 3.0, "Walls")
    }
}