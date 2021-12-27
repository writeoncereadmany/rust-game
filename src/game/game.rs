use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::world::world::World;
use crate::game_loop::GameLoop;
use crate::app::events::*;

pub struct Game<'a> {
    pub world: World<'a>
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, GEvent> for Game<'a> {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        self.world.event(event, events)
    }
}