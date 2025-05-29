use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};

use super::components::*;
use super::pickup::*;
use engine::audio::instrument::BELL;
use engine::audio::notes::*;
use engine::audio::tempo::Tempo;
use engine::shapes::shape::shape::Shape;

pub enum Fruit {
    APPLE,
    BANANA,
    CHERRY,
    GRAPES,
    WATERMELON
}

pub fn spawn_fruit(x: f64, y: f64, fruit: &Fruit, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Pickup)
        .with(Position(x, y))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
        .with(sprite(fruit))
        .with(OnPickupEffect::Sparkles)
        .with(OnPickupDo::CollectFruit(20))
    );
}

fn sprite(fruit: &Fruit) -> Sprite {
    match fruit {
        Fruit::APPLE => Sprite::new(6, 4, 0.0, "Sprites"),
        Fruit::BANANA => Sprite::new(5, 4, 0.0, "Sprites"),
        Fruit::CHERRY => Sprite::new(4, 4, 0.0, "Sprites"),
        Fruit::GRAPES => Sprite::new(4, 3, 0.0, "Sprites"),
        Fruit::WATERMELON => Sprite::new(7, 4, 0.0, "Sprites")
    }
}