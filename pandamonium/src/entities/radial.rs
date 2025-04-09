use std::time::Duration;
use std::f64::consts::PI;
use component_derive::{Constant, Variable};
use engine::events::{Event, Events};
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};
use crate::app::events::Destroy;
use crate::entities::components::{AnimationCycle, Period, Phase, Position};

#[derive(Clone, Variable)]
struct Radius(f64);

#[derive(Clone, Constant)]
struct AngleOffset(f64);

#[derive(Clone, Variable)]
struct Angle(f64);

#[derive(Clone, Variable)]
struct Center(f64, f64);

pub fn spawn_radial(x: f64, y: f64, r: f64, theta: f64, entities: &mut Entities, events: &mut Events) {
    let radial_id = entities.spawn(entity()
        .with(Center(x, y))
        .with(Radius(r))
        .with(Sprite::new(4, 2, 3.0, "Sprites"))
        .with(Period(0.6))
        .with(Phase(0.0))
        .with(AngleOffset(theta))
    );

    events.schedule(Duration::from_millis(2400), Destroy(radial_id));
}

pub fn radial_events(event: &Event, entities: &mut Entities, _events: &mut Events)
{
    event.apply(|dt| rotate(entities, dt));
}

pub fn rotate(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(Phase(phase), AngleOffset(theta))| Angle((phase * 2.0 * PI) + theta));
    entities.apply(|(Center(x, y), Angle(theta), Radius(r))| Position(x + (r * f64::sin(theta)), y + (r * f64::cos(theta))));
}
