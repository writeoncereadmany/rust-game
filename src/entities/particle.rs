use std::time::Duration;

use super::components::*;
use crate::events::Events;
use crate::app::events::Destroy;
use entity::{ Entities, entity };

pub fn spawn_particle(x: f64, y: f64, entities: &mut Entities, events: &mut Events) {
    let spangle_id = entities.spawn(entity()
        .with(FixedPosition(x, y))
        .with(Tile((0, 4)))
        .with(Period(0.45))
        .with(Phase(0.0))
        .with(AnimationCycle(vec![(0.33, (0,4)), (0.66, (1, 4)), (1.0, (0,4))]))
    );

    events.schedule(Duration::from_millis(450), Destroy(spangle_id));
}