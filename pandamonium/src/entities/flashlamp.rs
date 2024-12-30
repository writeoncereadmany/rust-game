use component_derive::{Event};
use engine::events::{Event, EventTrait, Events};
use crate::entities::components::Position;
use engine::graphics::sprite::Sprite;
use entity::{entity, Entities};

#[derive(Event)]
pub struct LightFlashbulb(pub u64);

pub fn spawn_flashlamp(x: f64, y: f64, entities: &mut Entities) -> u64 {
    entities.spawn(
        entity()
            .with(Position(x, y))
            .with(Sprite::new(6, 4, 0.0, "Walls")),
    )
}

pub fn flashbulb_events(entities: &mut Entities, event: &Event, _events: &mut Events) {
    event.apply(|LightFlashbulb(entity_id)| entities.apply_to(entity_id, |()| Sprite::new(7, 4, 0.0, "Walls")));
}