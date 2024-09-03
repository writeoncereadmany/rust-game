use std::time::Duration;

use entity::{ entity, Entities };
use engine::graphics::sprite::Sprite;
use engine::audio::audio::PlayTune;
use engine::audio::instrument::*;
use engine::audio::notes::C4;
use engine::shapes::shape::shape::Shape;
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
        .with(OnPickupTune(PlayTune(3, vec![
            (Duration::from_millis(0), CYMBAL.note(C4, 0.5)),
        ])))
        .with(OnPickupDo::OpenChests)
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
    );
}