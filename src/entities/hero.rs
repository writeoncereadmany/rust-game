use std::time::Duration;

use crate::controller::Controller;
use crate::game_loop::GameLoop;
use crate::graphics::renderer::{Layer, Renderer};
use crate::app::events::*;
use crate::shapes::convex_mesh::ConvexMesh;

const ACCEL: f64 = 600.0;
const REVERSE_ACCEL: f64 = 1200.0;
const STOPPING_SPEED: f64 = 50.0;
const VEL_CAP: f64 = 200.0;
const WALLJUMP_DY: f64 = 220.0;
const WALLJUMP_DX: f64 = 200.0;
const WALL_STICK: f64 = 10.0;
const JUMP_SPEED: f64 = 250.0;
const GRAVITY: f64 = 2000.0;
const EXTRA_JUMP: f64 = 1500.0;
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
    pub fn new(x: f64, y: f64, width: u32, height: u32, controller: Controller) -> Self {
        Hero {
            controller,
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            last_push: (0.0, 0.0),
            extrajump: 0.0,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (width as f64, 0.0), (width as f64, height as f64), (0.0, height as f64)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>, GEvent> for Hero {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_tile(&Layer::FOREGROUND, (0, 0), self.x, self.y);
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

    if hero.dx == 0.0 {
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