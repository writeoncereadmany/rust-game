use image::RgbImage;

use crate::graphics::renderer::{ Renderer, align };
use crate::world::world::World;
use crate::game_loop::GameLoop;
use crate::app::events::*;

pub struct Game<'a> {
    pub levels: &'a Vec<RgbImage>,
    pub world: World,
    pub level: usize,
    pub score: u32,
}

impl <'a> GameLoop<'a, Renderer<'a>, GEvent> for Game<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        self.world.render(renderer)?;
        renderer.draw_text(
            self.score.to_string(), 
            3.0, 
            17.5, 
            align::RIGHT & align::MIDDLE);
        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match event {
            Event::Game(GEvent::CoinCollected(_)) => self.score += 10,
            Event::Game(GEvent::TimeLimitExpired) => {
                self.world = World::new(&self.levels[self.level], self.world.hero.controller)
            },
            Event::Game(GEvent::ReachedDoor) => {
                self.level = (self.level + 1) % self.levels.len();
                self.world = World::new(&self.levels[self.level], self.world.hero.controller);
            }
            _ => { }
        }
        self.world.event(event, events)
    }
}