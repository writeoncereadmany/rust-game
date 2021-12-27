use std::time::Duration;

use sdl2::event::Event;

use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::world::world::World;
use crate::game_loop::GameLoop;

pub struct Game<'a> {
    pub world: World<'a>
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, f64> for Game<'a> {

    fn update(&mut self, dt: &Duration) -> Result<(), String> {
        self.world.update(dt)
    }

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.world.on_event(event)
    }
}