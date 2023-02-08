use std::time::Duration;

use entity::{ entity, Entities };

use crate::audio::audio::*;
use crate::audio::instrument::*;
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
            (Duration::from_millis(0), FLUTE.note(A, 3, 0.1)),
            (Duration::from_millis(30), FLUTE.note(Bb, 3, 0.1)),
            (Duration::from_millis(60), FLUTE.note(B, 3, 0.1)),
            (Duration::from_millis(90), FLUTE.note(C, 3, 0.1)),
            (Duration::from_millis(120), FLUTE.note(Db, 3, 0.1)),
            (Duration::from_millis(150), FLUTE.note(D, 3, 0.1)),
            (Duration::from_millis(180), FLUTE.note(Eb, 3, 0.1)),
            (Duration::from_millis(210), FLUTE.note(E, 3, 0.1)),
            (Duration::from_millis(240), FLUTE.note(F, 3, 0.1)),
            (Duration::from_millis(270), FLUTE.note(Gb, 3, 0.1)),
            (Duration::from_millis(300), FLUTE.note(G, 3, 0.1)),
            (Duration::from_millis(330), FLUTE.note(Ab, 4, 0.1)),
            (Duration::from_millis(360), FLUTE.note(A, 4, 0.1)),
        ]))
        .with(OnPickupDo::CompleteLevel)
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}