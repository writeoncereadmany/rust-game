use std::time::Duration;

use component_derive::Variable;
use engine::events::Events;
use engine::graphics::renderer::{align, Text};

use super::components::Position;
use crate::app::events::TimeLimitExpired;
use entity::*;

const LEVEL_TIME : f64 = 10.0;

#[derive(Clone, Variable)]
pub struct RemainingTime(f64);

#[derive(Clone, Variable)]
pub struct RemainingTimePercent(f64);

const CENTRED: u8 = align::CENTER & align::MIDDLE;

pub fn spawn_timer(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x, y))
        .with(Text { text: "".to_string(), justification: CENTRED })
        .with(RemainingTime(LEVEL_TIME))
        .with(RemainingTimePercent(100.0))
    );
}

pub fn update_timer(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    entities.apply(|RemainingTime(rt)| RemainingTime(rt - dt.as_secs_f64()));
    entities.apply(|RemainingTime(rt)| if rt <= 0.0 { events.fire(TimeLimitExpired) });
    entities.apply(|RemainingTime(rt)| Text { text: format!("{:01}", (rt * 10.0) as u32), justification: CENTRED });
    entities.apply(|RemainingTime(rt)| RemainingTimePercent((rt / LEVEL_TIME) * 100.0));
}