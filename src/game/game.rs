use std::time::Duration;

use sdl2::event::Event;

use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::world::world::World;
use crate::game_loop::GameEvents;

pub struct Game<'a> {
    pub world: World<'a>
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for Game<'a> {

    fn update(&mut self, dt: Duration) -> Result<(), String> {
        self.world.update(dt)
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.world.on_event(event)
    }
}