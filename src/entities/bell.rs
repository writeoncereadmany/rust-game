use std::time::Duration;

use component_derive::Constant;
use entity::{ entity, Component, Entities };

use crate::app::events::*;
use crate::audio::audio::*;
use crate::graphics::sprite::Sprite;
use crate::events::Events;
use crate::shapes::convex_mesh::ConvexMesh;
use super::components::*;

#[derive(Constant)]
pub struct Bell;

pub fn spawn_bell(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Bell)
        .with(Position(x, y))
        .with(Sprite::new(1, 0, 0.5))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}

pub fn collect_bell(&BellCollected { x, y, id }: &BellCollected, events: &mut Events) {
    events.fire(Destroy(id));
    events.fire(SpawnParticle(x, y));
    events.fire(SpawnText(x + 0.5, y + 1.5, "x2".to_string()));
    events.fire(PlayTune(vec![
        (Duration::from_millis(0), Note::Wave { pitch: B * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.3, 0.0)]) }),
        (Duration::from_millis(60), Note::Wave { pitch: E * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
        (Duration::from_millis(120), Note::Wave { pitch: B * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.3, 0.0)]) }),
    ]));
}