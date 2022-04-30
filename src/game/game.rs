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

    fn event(&mut self, event: &Eventy, events: &mut Events) -> Result<(), String> {
        if let Some(eventu) = event.unwrap()
        {
            match eventu {
                GEvent::CoinCollected(_) => self.score += 10,
                GEvent::TimeLimitExpired => {
                    self.world = World::new(&self.levels[self.level], self.world.hero.controller, other_type(&self.world.hero.panda_type))
                },
                GEvent::ReachedDoor => {
                    self.level = (self.level + 1) % self.levels.len();
                    self.world = World::new(&self.levels[self.level], self.world.hero.controller, other_type(&self.world.hero.panda_type));
                }
                _ => { }
            }
        }
        self.world.event(event, events)
    }
}