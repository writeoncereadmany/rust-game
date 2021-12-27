use std::time::Duration;


use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::world::world::World;
use crate::game_loop::*;

pub struct Game<'a> {
    pub world: World<'a>
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, f64> for Game<'a> {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.world.render(renderer)
    }

    fn event(&mut self, event: &Event<f64>, events: &mut Events<f64>) -> Result<(), String> {
        self.world.event(event, events)
    }
}