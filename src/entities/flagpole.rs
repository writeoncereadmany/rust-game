use std::time::Duration;

use entity::{ entity, Component, Entities };

use component_derive::Constant;

use crate::app::events::*;
use crate::audio::audio::*;
use crate::events::Events;
use crate::shapes::convex_mesh::ConvexMesh;
use crate::graphics::sprite::Sprite;

use super::components::*;

#[derive(Clone, Constant)]
pub struct Flagpole;

pub fn spawn_flagpole(x: f64, y: f64, entities: &mut Entities) {
    let animation_cycle = AnimationCycle(vec!(
        (0.5, Sprite::new(0, 1, 0.5)), 
        (1.0, Sprite::new(1, 1, 0.5)))); 
    entities.spawn(entity()
        .with(Flagpole)
        .with(Position(x, y))
        .with(Sprite::new(0, 1, 0.5))
        .with(animation_cycle)
        .with(Period(0.2))
        .with(Phase(0.0))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
    );
}

pub fn spawn_empty_flagpole(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Position(x,y))
        .with(Sprite::new(5, 7, 0.5)));
}

pub fn collect_flag(&FlagpoleCollected { id }: &FlagpoleCollected, entities: &mut Entities, events: &mut Events)
{
    if let Some(Position(x, y)) = entities.delete(&id)
    {
        spawn_empty_flagpole(x, y, entities);
        events.fire(PlayTune(vec![
            (Duration::from_millis(0), Note::Wave { pitch: A * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.3, 0.0)]) }),
            (Duration::from_millis(30), Note::Wave { pitch: Bb * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(60), Note::Wave { pitch: B * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(90), Note::Wave { pitch: C * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(120), Note::Wave { pitch: Db * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(150), Note::Wave { pitch: D * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(180), Note::Wave { pitch: Eb * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(210), Note::Wave { pitch: E * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(240), Note::Wave { pitch: F * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(270), Note::Wave { pitch: Gb * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(300), Note::Wave { pitch: G * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(330), Note::Wave { pitch: Ab * 2.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
            (Duration::from_millis(360), Note::Wave { pitch: A * 4.0, envelope: EnvSpec::vols(vec![(0.0, 0.25), (0.5, 0.0)]) }),
        ]));
        events.fire(ReachedDoor);   
    }
}