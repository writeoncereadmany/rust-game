use std::time::Duration;

use entity::{ entity, Entities };

use crate::audio::audio::*;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;
use super::components::*;
use super::pickup::OnPickupDo;
use super::pickup::OnPickupEffect;
use super::pickup::OnPickupTune;
use super::pickup::Pickup;

pub fn spawn_key(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(Sprite::new(4, 7, 0.5))
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupTune(vec![
            (Duration::from_millis(0), Note::Wave { pitch: B * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.3, 0.0)]) }),
            (Duration::from_millis(60), Note::Wave { pitch: E * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(120), Note::Wave { pitch: B * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.3, 0.0)]) }),
        ]))
        .with(OnPickupDo::OpenChests)
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}