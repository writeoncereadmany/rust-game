use std::time::Duration;

use crate::controller::Controller;
use crate::game_loop::GameLoop;
use crate::graphics::renderer::Renderer;
use crate::app::events::*;
use crate::shapes::convex_mesh::ConvexMesh;
use crate::sign::{ Sign, Signed };

const ACCEL: f64 = 30.0;
const REVERSE_ACCEL: f64 = 60.0;
const AIR_ACCEL: f64 = 20.0;
const AIR_SLOWDOWN: f64 = 10.0;
const STOPPING_SPEED: f64 = 1.0;
const VEL_CAP: f64 = 15.0;
const WALLJUMP_DY: f64 = 12.0;
const WALLJUMP_DX: f64 = 12.0;
const WALL_STICK: f64 = 0.1;
const JUMP_SPEED: f64 = 15.0;
const GRAVITY: f64 = 100.0;
const EXTRA_JUMP: f64 = 90.0;
const EXTRA_JUMP_DURATION: f64 = 0.215;

const UNITS_PER_FRAME: f64 = 1.0;
const RUN_CYCLE : [(i32, i32); 4] = [(1, 2), (2, 2), (3, 2), (2, 2)];
const ASCENDING : (i32, i32) = (2, 1);
const DESCENDING : (i32, i32) = (3, 1);


pub struct Hero {
    pub controller: Controller,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub last_push: (f64, f64),
    facing: Sign,
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
            facing: Sign::POSITIVE,
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
        let (_, last_push_y) = self.last_push;
        let tile = if last_push_y == 0.0 {
            if self.dy > 0.0 { 
                ASCENDING
            } else {
                DESCENDING
            }
        } else if self.dx.abs() < STOPPING_SPEED {
            (0, 2)
        } else {
            let frame: usize = (self.x / UNITS_PER_FRAME) as usize % RUN_CYCLE.len();
            RUN_CYCLE[frame]
        };
        let flip_x = self.facing == Sign::NEGATIVE;
        renderer.draw_tile_ex(tile, self.x, self.y, flip_x, false);
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
    let (last_push_x, last_push_y) = hero.last_push;
    let airborne = last_push_y <= 0.0;
    let x_vel_sign : Sign = hero.dx.sign();
    let x_input : Sign = hero.controller.x();

    if x_input == Sign::ZERO {

        if airborne {
            hero.dx -= AIR_SLOWDOWN * x_vel_sign.unit_f64() * dt;
        } else {
            hero.dx -= REVERSE_ACCEL * x_vel_sign.unit_f64() * dt;
        }

        if hero.dx.abs() < STOPPING_SPEED {
            hero.dx = 0.0;
        }
    } else {

        if airborne {
            hero.dx += AIR_ACCEL * x_input.unit_f64() * dt;
        } else if x_input == x_vel_sign {
            hero.dx += ACCEL * x_input.unit_f64() * dt;
        } else {
            hero.dx += REVERSE_ACCEL * x_input.unit_f64() * dt;
        }

    }
        
    hero.dx = hero.dx.clamp(-VEL_CAP, VEL_CAP);

    match hero.dx.sign() {
        Sign::POSITIVE => hero.facing = Sign::POSITIVE,
        Sign::NEGATIVE => hero.facing = Sign::NEGATIVE,
        Sign::ZERO => {}
    }

    if hero.controller.x() == Sign::ZERO {
        match last_push_x.sign() {
            Sign::POSITIVE => {
                hero.dx -= WALL_STICK;
            }
            Sign::NEGATIVE => {
                hero.dx += WALL_STICK;
            }
            Sign::ZERO => {}
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