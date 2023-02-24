use entity::{ entity, Entities };

use crate::audio::notes::*;
use crate::audio::instrument::BELL;
use crate::audio::tempo::Tempo;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;
use super::components::*;
use super::pickup::*;

pub fn spawn_bell(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(Sprite::new(1, 0, 0.5))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupText("x2"))
        .with(OnPickupDo::DoubleScore)
        .with(OnPickupTune(Tempo::new(2, 250).using(&BELL, 3).play(1.0, 0.25, B3).play(1.25, 0.25, E4).play(1.5, 0.5, B3).build()))
    );
}