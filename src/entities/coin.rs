use entity::{ entity, Entities, };

use crate::audio::notes::*;
use crate::audio::instrument::BELL;
use crate::audio::tempo::Tempo;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;
use super::components::*;
use super::pickup::*;

pub fn spawn_coin(x: f64, y: f64, entities: &mut Entities) {
    let phase = phase_offset(x, y);
    let animation_cycle = AnimationCycle(vec!(
        (0.25, Sprite::new(0, 3, 0.5)), 
        (0.5, Sprite::new(1, 3, 0.5)), 
        (0.75, Sprite::new(2, 3, 0.5)), 
        (1.0, Sprite::new(3, 3, 0.5))));
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(next_frame(phase, &animation_cycle))
        .with(Period(0.7))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(Phase(phase_offset(x, y)))
        .with(animation_cycle)
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupTune(Tempo::new(2, 250).using(&BELL, 3).play(1.0, 0.25, B5).play(1.25, 1.0, E6).build()))
        .with(OnPickupDo::Score(5))
    );
}

fn phase_offset(x: f64, y: f64) -> f64 {
    // magic numbers which don't mean anything, but feel good
    x * 0.8 + y * 0.4
}