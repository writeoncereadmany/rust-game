use std::time::Duration;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::controller::Controller;
use crate::game_loop::GameEvents;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::app::assets::Assets;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;

const ACCEL: f64 = 200.0;
const VEL_CAP: f64 = 200.0;
const JUMP_SPEED: f64 = 200.0;
const WALLJUMP_DY: f64 = 180.0;
const WALLJUMP_DX: f64 = 150.0;
const WALL_STICK: f64 = 10.0;
const GRAVITY: f64 = 500.0;

pub struct Hero<'a> {
    pub controller: Controller,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub sprite: Sprite<'a>,
    pub last_push: (f64, f64),
    mesh: ConvexMesh
}

impl <'a> Hero<'a> {  
    pub fn new(x: f64, y: f64, width: u32, height: u32, assets: &'a Assets<'a>) -> Self {
        Hero {
            controller: Controller::new(Keycode::Z, Keycode::X, Keycode::RShift),
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            sprite: assets.sprite(0, 0),
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

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for Hero<'a> {

    fn update(&mut self, dt: Duration) -> Result<(), String> {
        self.dx += self.controller.x() as f64 * ACCEL * dt.as_secs_f64();            
        self.dx = cap(self.dx, VEL_CAP);
        if self.controller.x() == 0 {
            match self.last_push {
                (x, _) if x > 0.0 => {
                    self.dx -= WALL_STICK;
                }
                (x, _) if x < 0.0 => {
                    self.dx += WALL_STICK;
                }
                _ => {}
            }
        }
        self.x += self.dx * dt.as_secs_f64();
        
        if self.controller.jump() {
            match self.last_push {
                (_, y) if y > 0.0 => { self.dy = JUMP_SPEED; },
                (x, _) if x > 0.0 => { 
                    self.dy = WALLJUMP_DY;
                    self.dx = WALLJUMP_DX;
                },
                (x, _) if x < 0.0 => {
                    self.dy = WALLJUMP_DY;
                    self.dx = -WALLJUMP_DX;
                }
                _ => {} 
            }
        }
        self.dy -= GRAVITY * dt.as_secs_f64();
        self.dy = cap(self.dy, VEL_CAP);

        self.y += self.dy * dt.as_secs_f64();

        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.draw(&Layer::FOREGROUND, &self.sprite, self.x as i32, self.y as i32);
        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.controller.on_event(event);
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