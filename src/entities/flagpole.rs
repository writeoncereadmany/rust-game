use std::time::Duration;

use entity::{ entity, Entities };

use crate::audio::audio::*;
use crate::audio::instrument::BELL;
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
        .with(OnPickupTune(vec![
            (Duration::from_millis(0), BELL.note(A, 3, 0.0)),
            (Duration::from_millis(30), BELL.note(Bb, 3, 0.0)),
            (Duration::from_millis(60), BELL.note(B, 3, 0.0)),
            (Duration::from_millis(90), BELL.note(C, 3, 0.0)),
            (Duration::from_millis(120), BELL.note(Db, 3, 0.0)),
            (Duration::from_millis(150), BELL.note(D, 3, 0.0)),
            (Duration::from_millis(180), BELL.note(Eb, 3, 0.0)),
            (Duration::from_millis(210), BELL.note(E, 3, 0.0)),
            (Duration::from_millis(240), BELL.note(F, 3, 0.0)),
            (Duration::from_millis(270), BELL.note(Gb, 3, 0.0)),
            (Duration::from_millis(300), BELL.note(G, 3, 0.0)),
            (Duration::from_millis(330), BELL.note(Ab, 4, 0.0)),
            (Duration::from_millis(360), BELL.note(A, 4, 0.0)),
        ]))
        .with(OnPickupDo::CompleteLevel)
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}