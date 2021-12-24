use std::time::Duration;

use sdl2::event::Event;

use crate::shapes::push::Push;
use crate::entities::coin::Coin;
use crate::entities::hero::Hero;
use crate::map::Map;
use crate::shapes::convex_mesh::Meshed;
use crate::game_loop::GameEvents;
use crate::graphics::lo_res_renderer::{ Layer, LoResRenderer };
use crate::graphics::text_renderer::SpriteFont;

#[derive(Clone)]
pub enum Tile {
    STONE
}

pub struct World<'a> {
    pub ball: Hero<'a>,
    pub coins: Vec<Coin<'a>>,
    pub map: Map<Meshed<Tile>>,
    pub spritefont: &'a SpriteFont<'a>,
    pub time: f64,
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for World<'a> {
    
    fn update(&mut self, dt: Duration) -> Result<(), String> {
        
        self.ball.update(dt)?;

        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        for (_pos, t) in self.map.overlapping(&self.ball.mesh().bbox()) {
            let push = t.mesh.push(&self.ball.mesh());
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 && x.signum() == -self.ball.dx.signum() {
                        self.ball.x += x;
                        tot_x_push += x;
                        self.ball.dx = 0.0;
                    }
                    if y != 0.0 && y.signum() == -self.ball.dy.signum() {
                        self.ball.y += y;
                        tot_y_push += y;
                        self.ball.dy = 0.0;
                    }
                }
            }
        }
        self.ball.last_push = (tot_x_push, tot_y_push);

        let ball_mesh = self.ball.mesh();
        self.coins.retain(|coin| !ball_mesh.bbox().touches(&coin.mesh().bbox()));

        self.time -= dt.as_secs_f64();
        
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result <(), String> {
        for coin in self.coins.iter_mut() {
            coin.render(renderer)?;
        }
        self.ball.render(renderer)?;

        self.spritefont.render(time_units(self.time), 12*15 + 4, 12 * 17 + 2, renderer, &Layer::FOREGROUND);

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.ball.on_event(event)
    }
}

fn time_units(time: f64) -> String {
    format!("{:02}", (time * 10.0) as u32)
}
