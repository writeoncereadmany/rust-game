use std::time::Duration;

use crate::app::assets::Assets;
use crate::graphics::renderer::{ Renderer, Text, align };
use crate::world::world::World;
use crate::game_loop::*;
use crate::events::*;
use crate::audio::*;
use crate::app::events::*;
use crate::entities::hero::PandaType;
use crate::controller::Controller;

pub struct Game<'a> {
    pub assets: &'a Assets<'a>,
    pub controller: Controller,
    pub world: World,
    pub level: usize,
    pub score: u32,
}

impl <'a> GameLoop<'a, Renderer<'a>> for Game<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        self.world.render(renderer)?;
        renderer.draw_text(
            &Text { text: self.score.to_string(), justification: align::RIGHT & align::MIDDLE}, 
            3.0, 
            14.5);
        Ok(())
    }

    fn event(&mut self, event: &Event, mut events: &mut Events) -> Result<(), String> {
        self.controller.on_event(event, &mut events);
        event.apply(|CoinCollected { .. }| {
            self.score += 10;
            events.fire(PlayTune(vec![
                (Duration::from_millis(0), Note::Wave { pitch: B * 4.0, volume: 0.05, length: Duration::from_millis(200) }),
                (Duration::from_millis(60), Note::Wave { pitch: E * 4.0, volume: 0.05, length: Duration::from_millis(250) })
            ]));
        });
        event.apply(|TimeLimitExpired| {
            self.world = World::new(
                &self.assets,
                self.level, 
                PandaType::RedPanda,
                &mut events)
        });
        event.apply(|ReachedDoor| {
            self.level = (self.level + 1) % self.assets.levels.len();
            self.world = World::new(
                &self.assets,
                self.level, 
                PandaType::GiantPanda,
                &mut events);
        });
        self.world.event(event, events)
    }
}