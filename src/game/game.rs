use crate::graphics::renderer::{ Layer, Renderer, Justification };
use crate::app::assets::Assets;
use crate::world::world::World;
use crate::game_loop::GameLoop;
use crate::app::events::*;

pub struct Game<'a> {
    pub assets: &'a Assets<'a>,
    pub world: World,
    pub level: usize,
    pub score: u32,
}

impl <'a> GameLoop<'a, Renderer<'a, Layer>, GEvent> for Game<'a> {

    fn render(&self, renderer: &mut Renderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)?;
        renderer.draw_text(
            self.score.to_string(), 
            &Layer::FOREGROUND, 
            8 * 3 + 2, 
            12 * 17 + 2, 
            Justification::RIGHT);
        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match event {
            Event::Game(GEvent::CoinCollected(_)) => self.score += 10,
            Event::Game(GEvent::TimeLimitExpired) => self.world = World::new(&self.assets.level,  self.level),
            Event::Game(GEvent::ReachedDoor) => {
                self.level = (self.level + 1) % self.assets.level.len();
                self.world = World::new(&self.assets.level, self.level);
            }
            _ => { }
        }
        self.world.event(event, events)
    }
}