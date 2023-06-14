use std::time::Duration;

use component_derive::Variable;
use engine::graphics::renderer::{ Text, align };
use engine::events::Events;

use super::components::Position;
use crate::app::events::TimeLimitExpired;
use entity::*;

#[derive(Clone, Variable)]
pub struct RemainingTime(f64);

const CENTRED: u8 = align::CENTER & align::MIDDLE;

pub fn spawn_timer(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x, y))
        .with(Text {text: "".to_string(), justification: CENTRED })
        .with(RemainingTime(10.0))
    );
}

pub fn update_timer(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    entities.apply(|RemainingTime(rt)| RemainingTime(rt - dt.as_secs_f64()));
    entities.apply(|RemainingTime(rt)| if rt <= 0.0 { events.fire(TimeLimitExpired )});
    entities.apply(|RemainingTime(rt)| Text{ text: format!("{:01}", (rt * 10.0) as u32), justification: CENTRED });
}