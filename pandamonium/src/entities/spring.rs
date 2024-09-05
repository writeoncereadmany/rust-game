use std::time::Duration;

use component_derive::{ Constant, Variable };
use entity::{ entity, Entities };
use engine::graphics::sprite::Sprite;
use engine::events::{ Event, Events };
use engine::shapes::shape::shape::Shape;
use crate::app::events::Interaction;

use super::components::*;

const TOTAL_SPRING_TIME : f64 = 0.7;

#[derive(Clone, Variable)]
struct SinceLastTrigger(f64);

#[derive(Clone, Constant)]
struct Spring;

pub fn spawn_spring(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Spring)
        .with(Position(x, y))
        .with(Interacts::Spring)
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
        .with(Sprite::new(0, 8, 0.7, "Sprites"))
    );
}

pub fn spring_events(event: &Event, entities: &mut Entities, _events: &mut Events) {
    event.apply(|&Interaction { interaction_type, other_id, .. }| {
        if interaction_type == Interacts::Spring {
            entities.apply_to(&other_id, |last_trigger| {
                if let Some(existing@SinceLastTrigger(_)) = last_trigger {
                    existing                    
                } else {
                    SinceLastTrigger(0.0)
                }
            })
        }
    });

    event.apply(|dt: &Duration| {
        entities.apply(|SinceLastTrigger(t)| { let new_t = t + dt.as_secs_f64();
            if new_t <= TOTAL_SPRING_TIME { Some(SinceLastTrigger(new_t)) } else { None } 
        });
        entities.apply(|(Spring, last_trigger)| animate_spring(last_trigger));
    });
}

fn animate_spring(last_trigger: Option<SinceLastTrigger>) -> Sprite {
    if let Some(SinceLastTrigger(t)) = last_trigger {
        if t < 0.05 {
            Sprite::new(1, 8, 0.7, "Sprites")
        } else if t < 0.15 {
            Sprite::new(2, 8, 0.7, "Sprites")
        } else if t < 0.3 {
            Sprite::new(1, 8, 0.7, "Sprites")
        } else if t < 0.5 {
            Sprite::new(2, 8, 0.7, "Sprites")
        } else {
            Sprite::new(1, 8, 0.7, "Sprites")
        }
    } else {
        Sprite::new(0, 8, 0.7, "Sprites")
    }
}