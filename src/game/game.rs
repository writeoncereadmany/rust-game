use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::graphics::text_renderer::{ Justification, SpriteFont };
use crate::app::assets::Assets;
use crate::world::world::World;
use crate::game_loop::GameLoop;
use crate::app::events::*;

pub struct Game<'a> {
    pub assets: &'a Assets<'a>,
    pub world: World<'a>,
    pub spritefont: &'a SpriteFont<'a>,
    pub level: usize,
    pub score: u32,
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, GEvent> for Game<'a> {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)?;
        self.spritefont.render(
            self.score.to_string(), 
            8 * 3 + 2, 
            12 * 17 + 2, 
            renderer, 
            &Layer::FOREGROUND, 
            Justification::RIGHT);
        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match event {
            Event::Game(GEvent::CoinCollected(_)) => self.score += 10,
            Event::Game(GEvent::TimeLimitExpired) => self.world = World::new(self.assets, self.level),
            Event::Game(GEvent::ReachedDoor) => {
                self.level = self.level + 1 % self.assets.level.len();
                self.world = World::new(self.assets, self.level);
            }
            _ => { }
        }
        self.world.event(event, events)
    }
}