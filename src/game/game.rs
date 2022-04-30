use image::RgbImage;

use crate::graphics::renderer::{ Renderer, align };
use crate::world::world::World;
use crate::game_loop::*;
use crate::app::events::*;
use crate::entities::hero::other_type;

pub struct Game<'a> {
    pub levels: &'a Vec<RgbImage>,
    pub world: World,
    pub level: usize,
    pub score: u32,
}

impl <'a> GameLoop<'a, Renderer<'a>> for Game<'a> {

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
        if let Some(GEvent::CoinCollected(_)) = event.unwrap() {
            self.score += 10;
        }

        if let Some(GEvent::TimeLimitExpired) = event.unwrap() {
            self.world = World::new(
                &self.levels[self.level], 
                self.world.hero.controller, 
                other_type(&self.world.hero.panda_type))
        }

        if let Some(GEvent::ReachedDoor) = event.unwrap() {
            self.level = (self.level + 1) % self.levels.len();
            self.world = World::new(
                &self.levels[self.level], 
                self.world.hero.controller, 
                other_type(&self.world.hero.panda_type));
        }
        self.world.event(event, events)
    }
}