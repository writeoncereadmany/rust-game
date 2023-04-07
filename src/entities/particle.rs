use std::time::Duration;

use super::components::*;
use super::hero::PandaType;
use crate::events::Events;
use crate::graphics::sprite::Sprite;
use crate::graphics::renderer::{align, Text};
use crate::app::events::Destroy;
use entity::{ Entities, entity };

pub fn spawn_spangle(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(0, 7, 0.5))
        .with(Period(0.45))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![(0.33, Sprite::new(0, 7, 0.5)), (0.66, Sprite::new(1, 7, 0.5)), (1.0, Sprite::new(0, 7, 0.5))]))
    );

    events.schedule(Duration::from_millis(450), Destroy(spangle_id));
}

pub fn spawn_text(x: f64, y: f64, text: &str, entities: &mut Entities, events: &mut Events) {
    let text_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Text { text: text.to_string(), justification: align::CENTER | align::MIDDLE})
        .with(Velocity(0.0, 2.0))
    );
    events.schedule(Duration::from_millis(600), Destroy(text_id));
}

pub fn spawn_bulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let bulb_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5, 2.0))
        .with(Period(0.6))
        .with(Phase(0.0))
//        .with(Flicker((x as u32 ^ y as u32) & 1 == 1))
        .with(AnimationCycle(vec![
            (0.10, Sprite::new(4, 5, 2.0)), 
            (0.20, Sprite::new(5, 5, 2.0)),
            (0.30, Sprite::new(6, 5, 2.0)),
            (0.40, Sprite::new(7, 5, 2.0)),
            (0.60, Sprite::new(6, 5, 2.0)),
            (0.80, Sprite::new(5, 5, 2.0)),
            (1.00, Sprite::new(4, 5, 2.0)),            
        ]))
    );

    events.schedule(Duration::from_millis(600), Destroy(bulb_id));
}


pub fn spawn_flashbulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let bulb_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5, 2.0))
        .with(Period(0.3))
        .with(Phase(0.0))
//        .with(Flicker(true))
        .with(AnimationCycle(vec![
            (0.15, Sprite::new(4, 5, 2.0)), 
            (0.3, Sprite::new(5, 5, 2.0)),
            (0.45, Sprite::new(6, 5, 2.0)),
            (0.6, Sprite::new(7, 5, 2.0)),
            (1.00, Sprite::new(-1, -1, 2.0)),            
        ]))
    );

    events.schedule(Duration::from_millis(1150), Destroy(bulb_id));
}

pub fn spawn_shadow(x: f64, y: f64, panda_type: PandaType, entities: &mut Entities, events: &mut Events) {
    let offset = match panda_type {
        PandaType::GiantPanda => 0,
        PandaType::RedPanda => 4
    };


    let shadow_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(offset + 0, 6, 1.0))
        .with(Period(0.4))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![
            (0.25, Sprite::new(offset + 0, 6, 2.0)), 
            (0.50, Sprite::new(offset + 1, 6, 2.0)),
            (0.75, Sprite::new(offset + 2, 6, 2.0)),
            (1.00, Sprite::new(offset + 3, 6, 2.0)),
        ]))
    );

    events.schedule(Duration::from_millis(2400), Destroy(shadow_id));
}
