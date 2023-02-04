use std::time::Duration;

use component_derive::Variable;
use entity::{ entity, Entities };

use crate::app::events::Interaction;
use crate::events::{ Event, Events };
use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;

use super::components::*;

const TOTAL_SPRING_TIME : f64 = 0.5;

#[derive(Clone, Variable)]
struct SinceLastTrigger(f64);

pub fn spawn_spring(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x, y))
        .with(Interacts::Spring)
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 0.2), (0.0, 0.2)], vec![]).translate(x, y)))
        .with(Sprite::new(0, 8, 0.7))
    );
}

pub fn spring_events(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|&Interaction { interaction_type, other_id, .. }| {
        if interaction_type == Interacts::Spring {
            entities.apply_to(&other_id, |()| {
                SinceLastTrigger(0.0)
            })
        }
    });

    event.apply(|dt: &Duration| {
        entities.apply(|SinceLastTrigger(t)| { let new_t = t + dt.as_secs_f64();
            if new_t > TOTAL_SPRING_TIME { Some(SinceLastTrigger(new_t)) } else { None } 
        });
        entities.apply(|last_trigger| animate_spring(last_trigger));
    });
}

fn animate_spring(last_trigger: Option<SinceLastTrigger>) -> Sprite {
    if let Some(SinceLastTrigger(t)) = last_trigger {
        Sprite::new(2, 8, 0.7)
    } else {
        Sprite::new(0, 8, 0.7)
    }
}