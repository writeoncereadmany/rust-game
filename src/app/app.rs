use std::time::Duration;

use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::game_loop::GameEvents;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::graphics::text_renderer::SpriteFont;
use crate::controller::Controller;
use crate::world::world::World;
use crate::fps_counter::FpsCounter;

const ACCEL: f64 = 200.0;
const VEL_CAP: f64 = 200.0;
const JUMP_SPEED: f64 = 200.0;
const WALLJUMP_DY: f64 = 180.0;
const WALLJUMP_DX: f64 = 150.0;
const WALL_STICK: f64 = 10.0;
const GRAVITY: f64 = 500.0;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub active_controller: Option<GameController>,
    pub world: World<'a>,
    pub fps_counter: FpsCounter,
    pub controller: Controller,
    pub spritefont: SpriteFont<'a>,
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for App<'a> {

    fn update(&mut self, dt: Duration) -> Result<(), String> {

        self.world.ball.dx += self.controller.x() as f64 * ACCEL * dt.as_secs_f64();            
        self.world.ball.dx = cap(self.world.ball.dx, VEL_CAP);
        if self.controller.x() == 0 {
            match self.world.ball.last_push {
                (x, _) if x > 0.0 => {
                    self.world.ball.dx -= WALL_STICK;
                }
                (x, _) if x < 0.0 => {
                    self.world.ball.dx += WALL_STICK;
                }
                _ => {}
            }
        }
        self.world.ball.x += self.world.ball.dx * dt.as_secs_f64();
        
        if self.controller.jump() {
            match self.world.ball.last_push {
                (_, y) if y > 0.0 => { self.world.ball.dy = JUMP_SPEED; },
                (x, _) if x > 0.0 => { 
                    self.world.ball.dy = WALLJUMP_DY;
                    self.world.ball.dx = WALLJUMP_DX;
                },
                (x, _) if x < 0.0 => {
                    self.world.ball.dy = WALLJUMP_DY;
                    self.world.ball.dx = -WALLJUMP_DX;
                }
                _ => {} 
            }
        }
        self.world.ball.dy -= GRAVITY * dt.as_secs_f64();
        self.world.ball.dy = cap(self.world.ball.dy, VEL_CAP);

        self.world.ball.y += self.world.ball.dy * dt.as_secs_f64();

        self.world.update(dt)
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.clear(&Layer::FOREGROUND).unwrap();
        renderer.draw(&Layer::FOREGROUND, &self.world.ball.sprite, self.world.ball.x as i32, self.world.ball.y as i32);

        self.spritefont.render(self.fps_counter.fps().to_string() + " fps", 2, 2, renderer, &Layer::FOREGROUND);      

        renderer.present()
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.controller.on_event(event);
        match event {
            Event::Quit {..} => return Err("Escape pressed: ending game".into()),
            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
            Event::ControllerDeviceAdded{ which, .. } => { 
                self.active_controller = self.game_controller_subsystem.open(*which).ok(); 
            }
            _ => {}
        }
        Ok(())
    }
}

fn cap(val: f64, max: f64) -> f64 {
    if val > max {
        max
    } else if val < -max {
        -max
    } else {
        val
    }
}