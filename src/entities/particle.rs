use std::time::Duration;

use crate::game_loop::*;
use crate::graphics::renderer::Renderer;

const FRAMES : [(i32, i32);3] = [(0, 4), (1, 4), (0, 4)];
const FRAME_DURATION : f64 = 0.15;

pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub id: u32,
    pub existed_for: Duration,
    pub frame: usize,
    pub expired: bool
}

impl Particle {  
    pub fn new(x: f64, y: f64, id: u32) -> Self {
        Particle {
            x,
            y,
            id,
            existed_for: Duration::ZERO,
            frame: 0,
            expired: false
        }
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for Particle {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        if !self.expired {
            renderer.draw_tile(FRAMES[self.frame], self.x, self.y);
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, _events: &mut Events) -> Result<(), String> {
        if let Some(dt) = event.unwrap() {
            self.existed_for = self.existed_for + *dt;
            self.frame = (self.existed_for.as_secs_f64() / FRAME_DURATION) as usize;
            self.expired = self.frame >= FRAMES.len();
        }
        Ok(())
    }
}