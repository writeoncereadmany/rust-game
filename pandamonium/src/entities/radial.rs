use std::time::Duration;
use std::f64::consts::PI;
use component_derive::{Constant, Variable};
use engine::events::{Event, Events};
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};
use crate::app::events::Destroy;
use crate::entities::components::{Age, AnimationCycle, Period, Phase, Position};

#[derive(Clone, Variable)]
struct Radius(f64);

#[derive(Clone, Constant)]
struct AngleOffset(f64);

#[derive(Clone, Variable)]
struct Angle(f64);

#[derive(Clone, Variable)]
struct Center(f64, f64);

pub fn spawn_radial(x: f64, y: f64, h_grid_pos: i32, theta: f64, entities: &mut Entities, events: &mut Events) {
    let radial_id = entities.spawn(entity()
        .with(Center(x, y))
        .with(Sprite::new(h_grid_pos, 2, 5.0, "Sprites"))
        .with(Period(0.6))
        .with(Phase(0.0))
        .with(AngleOffset(theta))
        .with(Age(0.0))
    );

    events.schedule(Duration::from_millis(2600), Destroy(radial_id));
}

pub fn radial_events(event: &Event, entities: &mut Entities, _events: &mut Events)
{
    event.apply(|dt: &Duration| rotate(entities));
    entities.apply(update_radius);
    entities.apply(update_period);
}

pub fn rotate(entities: &mut Entities) {
    entities.apply(|(Phase(phase), AngleOffset(theta))| Angle((phase * 2.0 * PI) + theta));
    entities.apply(|(Center(x, y), Angle(theta), Radius(r))| Position(x + (r * f64::sin(theta)), y + (r * f64::cos(theta))));
}

pub fn update_radius(Age(age): Age) -> Radius {
    if (age < 0.8) {
        let through_phase = 1.0 - (age / 0.8);
        Radius(3.0 + (20.0 * through_phase))
    } else if (age < 2.2) {
        Radius(3.0)
    } else {
        let through_phase = 1.0 - ((age - 2.2) / 0.2);
        Radius(3.0 * through_phase)
    }
}

pub fn update_period(Age(age): Age) -> Period {
    if (age < 0.8) {
        Period(1.2)
    } else if (age < 1.8) {
        let through_phase = ((age - 0.8) / 1.0);
        Period(1.2 - through_phase)
    } else {
        Period(1000.0)
    }
}
