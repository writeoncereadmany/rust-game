use std::time::Duration;

use crate::controller::Controller;
use crate::game_loop::GameLoop;
use crate::graphics::renderer::Renderer;
use crate::app::events::*;
use crate::shapes::convex_mesh::ConvexMesh;

const ACCEL: f64 = 20.0;
const REVERSE_ACCEL: f64 = 60.0;
const AIR_ACCEL: f64 = 10.0;
const STOPPING_SPEED: f64 = 1.0;
const VEL_CAP: f64 = 15.0;
const WALLJUMP_DY: f64 = 12.0;
const WALLJUMP_DX: f64 = 12.0;
const WALL_STICK: f64 = 0.1;
const JUMP_SPEED: f64 = 15.0;
const GRAVITY: f64 = 70.0;
const EXTRA_JUMP: f64 = 55.0;
const EXTRA_JUMP_DURATION: f64 = 0.20;

pub struct Hero {
    pub controller: Controller,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub last_push: (f64, f64),
    extrajump: f64,
    mesh: ConvexMesh
}

impl Hero {  
    pub fn new(x: f64, y: f64, controller: Controller) -> Self {
        Hero {
            controller,
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            last_push: (0.0, 0.0),
            extrajump: 0.0,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>, GEvent> for Hero {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_tile((2, 1), self.x, self.y);
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
    let dt = dt.as_secs_f64();
    let (_, last_push_y) = hero.last_push;
    let grounded = last_push_y > 0.0;

    if !grounded {
        hero.dx += hero.controller.x() as f64 * AIR_ACCEL * dt;
    }
    else if hero.dx == 0.0 {
        hero.dx += hero.controller.x() as f64 * ACCEL * dt;
    }
    else if hero.dx > 0.0 {
        if hero.controller.x() > 0 {
            hero.dx += ACCEL * dt;
        }
        else if hero.controller.x() < 0 {
            hero.dx -= REVERSE_ACCEL * dt;
        }
        else if hero.dx.abs() < STOPPING_SPEED {
            hero.dx = 0.0;
        }
        else {
            hero.dx -= REVERSE_ACCEL * dt;
        }
    }
    else {
        if hero.controller.x() < 0 {
            hero.dx -= ACCEL * dt;
        }
        else if hero.controller.x() < 0 {
            hero.dx += REVERSE_ACCEL * dt;
        }
        else if hero.dx.abs() < STOPPING_SPEED {
            hero.dx = 0.0;
        }
        else {
            hero.dx += REVERSE_ACCEL * dt;
        }
    }
    hero.dx += hero.controller.x() as f64 * ACCEL * dt;            
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
    
    if hero.controller.jump_pressed() {
        match hero.last_push {
            (_, y) if y > 0.0 => { 
                hero.dy = JUMP_SPEED; 
                hero.extrajump = EXTRA_JUMP_DURATION;
            },
            (x, _) if x > 0.0 => { 
                hero.dy = WALLJUMP_DY;
                hero.dx = WALLJUMP_DX;
                hero.extrajump = EXTRA_JUMP_DURATION;

            },
            (x, _) if x < 0.0 => {
                hero.dy = WALLJUMP_DY;
                hero.dx = -WALLJUMP_DX;
                hero.extrajump = EXTRA_JUMP_DURATION;

            }
            _ => {} 
        }
    }
    else if hero.extrajump > 0.0 && hero.controller.jump_held() {
        hero.dy += EXTRA_JUMP * dt;
        hero.extrajump -= dt;
    }
    else {
        hero.extrajump = 0.0;
    }
    hero.dy -= GRAVITY * dt;

    hero.x += hero.dx * dt;
    hero.y += hero.dy * dt;

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