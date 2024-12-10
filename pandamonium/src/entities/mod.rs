pub mod bell;
pub mod chest;
pub mod coin;
pub mod key;
pub mod components;
pub mod flagpole;
pub mod hero;
pub mod particle;
pub mod pickup;
pub mod spring;
pub mod timer;
pub mod lockbox;

use entity::Entities;

use engine::events::{Event, Events};
use crate::app::events::*;
use crate::controllers::physics::*;
use crate::entities::hero::clamp_hero;
use self::chest::chest_events;
use self::hero::hero_events;
use self::lockbox::lockbox_events;
use self::particle::*;
use self::pickup::collect_pickup;
use self::spring::spring_events;
use self::timer::spawn_timer;

pub fn entity_events(event: &Event, entities: &mut Entities, events: &mut Events)
{
    hero_events(entities, event, events);
    chest_events(event, entities, events);
    lockbox_events(event, entities, events);
    spawn_events(event, entities, events);
    spring_events(event, entities, events);
    event.apply(|dt| gravity(entities, dt));
    event.apply(|dt| integrate(entities, dt));
    event.apply(|dt| translate(entities, dt));
    clamp_hero(entities, event, events);

    event.apply(|Destroy(id)| entities.delete::<()>(id));
    event.apply(|pickup| { collect_pickup(pickup, entities, events) });
}

pub fn spawn_events(event: &Event, entities: &mut Entities, events: &mut Events) {
    event.apply(|&SpawnTimer(x, y)| spawn_timer(x, y, entities));
    event.apply(|&SpawnParticle(x, y)| spawn_spangle(x, y, entities, events));
    event.apply(|&SpawnText(x, y, ref text)| spawn_text(x, y, text, entities, events));
    event.apply(|&SpawnBulb(x, y)| spawn_bulb(x, y, entities, events));
    event.apply(|&SpawnFlashBulb(x, y)| spawn_flashbulb(x, y, entities, events));
}