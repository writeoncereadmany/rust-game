use std::time::Duration;

use sdl2::keyboard::Keycode;

use crate::controller::Controller;
use crate::game_loop::GameLoop;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::app::events::*;
use crate::shapes::convex_mesh::ConvexMesh;

const ACCEL: f64 = 200.0;
const VEL_CAP: f64 = 200.0;
const JUMP_SPEED: f64 = 200.0;
const WALLJUMP_DY: f64 = 180.0;
const WALLJUMP_DX: f64 = 150.0;
const WALL_STICK: f64 = 10.0;
const GRAVITY: f64 = 500.0;

pub struct Hero {
    pub controller: Controller,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub last_push: (f64, f64),
    mesh: ConvexMesh
}

impl Hero {  
    pub fn new(x: f64, y: f64, width: u32, height: u32) -> Self {
        Hero {
            controller: Controller::new(Keycode::Z, Keycode::X, Keycode::RShift),
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            last_push: (0.0, 0.0),
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, GEvent> for Hero {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.draw_tile(&Layer::FOREGROUND, (0, 0), self.x as i32, self.y as i32);
        Ok(())
    }

    fn event(&mut self, event: &Event, _events: &mut Events) -> Result<(), String> {
        match event {
            Event::Sdl(e) => self.controller.on_event(e),
            Event::Time(dt) => { update(self, dt)?; },
            _ => { }
        }
        Ok(())
    }
}

fn update(hero: &mut Hero, dt: &Duration) -> Result<(), String> {
    hero.dx += hero.controller.x() as f64 * ACCEL * dt.as_secs_f64();            
    hero.dx = cap(hero.dx, VEL_CAP);
    if hero.controller.x() == 0 {
        match hero.last_push {
            (x, _) if x > 0.0 => {
                hero.dx -= WALL_STICK;
            }
            (x, _) if x < 0.0 => {
                hero.dx += WALL_STICK;
            }
            _ => {}
        }
    }
    
    if hero.controller.jump() {
        match hero.last_push {
            (_, y) if y > 0.0 => { hero.dy = JUMP_SPEED; },
            (x, _) if x > 0.0 => { 
                hero.dy = WALLJUMP_DY;
                hero.dx = WALLJUMP_DX;
            },
            (x, _) if x < 0.0 => {
                hero.dy = WALLJUMP_DY;
                hero.dx = -WALLJUMP_DX;
            }
            _ => {} 
        }
    }
    hero.dy -= GRAVITY * dt.as_secs_f64();
    hero.dy = cap(hero.dy, VEL_CAP);

    hero.x += hero.dx * dt.as_secs_f64();
    hero.y += hero.dy * dt.as_secs_f64();

    Ok(())
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