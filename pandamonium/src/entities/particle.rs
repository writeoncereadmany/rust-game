use engine::events::Events;
use engine::graphics::renderer::{align, Text};
use engine::graphics::sprite::Sprite;
use std::time::Duration;

use super::components::*;
use super::hero::PandaType;
use crate::app::events::Destroy;
use entity::{entity, Entities};

pub fn spawn_spangle(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(0, 7, 0.5, "Sprites"))
        .with(Period(0.45))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![(0.33, Sprite::new(0, 7, 0.5, "Sprites")), (0.66, Sprite::new(1, 7, 0.5, "Sprites")), (1.0, Sprite::new(0, 7, 0.5, "Sprites"))]))
    );

    events.schedule("game", Duration::from_millis(450), Destroy(spangle_id));
}

pub fn spawn_text(x: f64, y: f64, text: &str, entities: &mut Entities, events: &mut Events) {
    let text_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Text { text: text.to_string(), justification: align::CENTER | align::MIDDLE })
        .with(Velocity(0.0, 2.0))
    );
    events.schedule("game", Duration::from_millis(600), Destroy(text_id));
}

pub fn spawn_bulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let bulb_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5, 2.0, "Sprites"))
        .with(Period(0.6))
        .with(Phase(0.0))
        //        .with(Flicker((x as u32 ^ y as u32) & 1 == 1))
        .with(AnimationCycle(vec![
            (0.10, Sprite::new(4, 5, 2.0, "Sprites")),
            (0.20, Sprite::new(5, 5, 2.0, "Sprites")),
            (0.30, Sprite::new(6, 5, 2.0, "Sprites")),
            (0.40, Sprite::new(7, 5, 2.0, "Sprites")),
            (0.60, Sprite::new(6, 5, 2.0, "Sprites")),
            (0.80, Sprite::new(5, 5, 2.0, "Sprites")),
            (1.00, Sprite::new(4, 5, 2.0, "Sprites")),
        ]))
    );

    events.schedule("game", Duration::from_millis(600), Destroy(bulb_id));
}


pub fn spawn_flashbulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let bulb_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5, 2.0, "Sprites"))
        .with(Period(0.3))
        .with(Phase(0.0))
        //        .with(Flicker(true))
        .with(AnimationCycle(vec![
            (0.15, Sprite::new(4, 5, 2.0, "Sprites")),
            (0.3, Sprite::new(5, 5, 2.0, "Sprites")),
            (0.45, Sprite::new(6, 5, 2.0, "Sprites")),
            (0.6, Sprite::new(7, 5, 2.0, "Sprites")),
            (1.00, Sprite::new(-1, -1, 2.0, "Sprites")),
        ]))
    );

    events.schedule("game", Duration::from_millis(1150), Destroy(bulb_id));
}
