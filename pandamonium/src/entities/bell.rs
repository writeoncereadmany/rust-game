use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};

use super::components::*;
use super::pickup::*;
use engine::audio::instrument::BELL;
use engine::audio::notes::*;
use engine::audio::tempo::Tempo;
use engine::shapes::shape::shape::Shape;

pub fn spawn_bell(x: f64, y: f64, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(Sprite::new(1, 0, 0.5, "Sprites"))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupText("x2"))
        .with(OnPickupDo::DoubleScore)
        .with(OnPickupTune(Tempo::new(2, 250).using(&BELL, 3).play(1.0, 0.25, B3).play(1.25, 0.25, E4).play(1.5, 0.5, B3).build()))
    );
}