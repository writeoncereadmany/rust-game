use component_derive::Event;
use engine::events::EventTrait;

use crate::entities::components::Interacts;
use crate::entities::hero::PandaType;

use super::app::HiScore;

#[derive(Event)]
pub struct NewGame(pub PandaType);

#[derive(Event)]
pub struct ShowHighScores();


#[derive(Event)]
pub struct ShowTitleScreen();

#[derive(Event)]
pub struct FlagpoleCollected {
    pub id: u64,
}

#[derive(Event)]
pub struct KeyCollected;

#[derive(Event)]
pub struct TimeLimitExpired;

#[derive(Event)]
pub struct UpdateHiScores(pub Vec<HiScore>);

#[derive(Event)]
pub struct ReachedDoor(pub String);

#[derive(Event)]
pub struct Destroy(pub u64);

#[derive(Event)]
pub struct SpawnParticle(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnText(pub f64, pub f64, pub String);

#[derive(Event)]
pub struct SpawnBulb(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnFlashBulb(pub f64, pub f64);

#[derive(Event)]
pub struct SpawnHero(pub f64, pub f64, pub PandaType);

#[derive(Event)]
pub struct ClearAudio();

#[derive(Event)]
pub struct GameOver(pub u32);

#[derive(Event)]
pub struct Interaction {
    pub hero_id: u64,
    pub other_id: u64,
    pub interaction_type: Interacts,
}

#[derive(Clone, Event)]
pub enum Score {
    Points(u32),
    Double,
}
