use std::time::Duration;

use super::components::*;
use crate::events::Events;
use crate::graphics::sprite::Sprite;
use crate::app::events::Destroy;
use entity::{ Entities, entity };

pub fn spawn_particle(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(0, 4))
        .with(Period(0.45))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![(0.33, Sprite::new(0,4)), (0.66, Sprite::new(1, 4)), (1.0, Sprite::new(0,4))]))
    );

    events.schedule(Duration::from_millis(450), Destroy(spangle_id));
}

pub fn spawn_bulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5))
        .with(Period(1.0))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![
            (0.10, Sprite::new(4, 5)), 
            (0.20, Sprite::new(5, 5)),
            (0.30, Sprite::new(6, 5)),
            (0.40, Sprite::new(7, 5)),
            (0.60, Sprite::new(6, 5)),
            (0.80, Sprite::new(5, 5)),
            (1.00, Sprite::new(4, 5)),            
        ]))
    );

    events.schedule(Duration::from_millis(900), Destroy(spangle_id));
}


pub fn spawn_flashbulb(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(Position(x, y))
        .with(Sprite::new(4, 5))
        .with(Period(0.3))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![
            (0.15, Sprite::new(4, 5)), 
            (0.3, Sprite::new(5, 5)),
            (0.45, Sprite::new(6, 5)),
            (0.6, Sprite::new(7, 5)),
            (1.00, Sprite::new(-1, -1)),            
        ]))
    );

    events.schedule(Duration::from_millis(850), Destroy(spangle_id));
}