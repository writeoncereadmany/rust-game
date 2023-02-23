use std::time::Duration;

use entity::{ entity, Entities };

use crate::audio::audio::*;
use crate::audio::instrument::*;
use crate::audio::tempo::Tempo;
use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;

use super::components::*;
use super::pickup::*;

pub fn spawn_flagpole(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x,y))
        .with(Sprite::new(5, 7, 0.3)));

    let animation_cycle = AnimationCycle(vec!(
        (0.5, Sprite::new(6, 7, 0.5)), 
        (1.0, Sprite::new(7, 7, 0.5)))); 

    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(Sprite::new(6, 7, 0.5))
        .with(animation_cycle)
        .with(Period(0.2))
        .with(Phase(0.0))
        .with(OnPickupTune(Tempo::new(4, 250).using(&FLUTE, 3)
            .play(1.0, 0.25, A, 3).play(1.25, 0.25, Cs, 3).play(1.5, 0.25, E, 3).play(1.75, 1.25, A, 4).build()))
        .with(OnPickupDo::CompleteLevel)
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}