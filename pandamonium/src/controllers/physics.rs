use std::time::Duration;
use entity::Entities;
use crate::entities::components::{Gravity, Position, Translation, Velocity};
const GRAVITY: f64 = 100.0;

pub fn gravity(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(Gravity, Velocity(dx, dy))| Velocity(dx, dy - GRAVITY * dt.as_secs_f64()))
}

pub fn integrate(entities: &mut Entities, dt: &Duration) {
    entities.apply(|Velocity(dx, dy)| Translation(dx * dt.as_secs_f64(), dy * dt.as_secs_f64()));
}

pub fn translate(entities: &mut Entities, _dt: &Duration) {
    entities.apply(|(Translation(tx, ty), Position(x, y))| Position(x + tx, y + ty));
}