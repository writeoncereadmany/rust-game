use std::time::Duration;

use component_derive::{ Constant, Variable };
use entity::{ entity, Entities, Component, Variable };

use crate::controller::{ ButtonPress, ControllerState };
use crate::game_loop::*;
use crate::events::*;
use crate::graphics::renderer::Renderer;
use crate::graphics::sprite::Sprite;
use crate::shapes::convex_mesh::ConvexMesh;
use crate::sign::{ Sign, Signed };
use super::components::*;

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

const PANDA_OFFSET: i32 = 1;
const RED_PANDA_OFFSET: i32 = 4;

const UNITS_PER_FRAME: f64 = 1.0;
const RUN_CYCLE : [(i32, i32); 4] = [(1, 1), (2, 1), (3, 1), (2, 1)];
const ASCENDING : (i32, i32) = (2, 0);
const DESCENDING : (i32, i32) = (3, 0);
const STANDING: (i32, i32) = (0, 1);

#[derive(Clone, Copy, Constant)]
pub enum PandaType {
    GiantPanda,
    RedPanda
}

#[derive(Constant)]
pub struct Heroo;

#[derive(Variable)]
pub struct MovingX(pub Sign);

#[derive(Variable)]
pub struct Ascending(pub f64);

#[derive(Variable)]
pub struct LastPush(pub f64, pub f64);

#[derive(Variable)]
pub struct Facing(Sign);

pub fn other_type(panda_type: &PandaType) -> PandaType {
    match panda_type {
        PandaType::GiantPanda => PandaType::RedPanda,
        PandaType::RedPanda => PandaType::GiantPanda
    }
}

pub fn spawn_hero(x: f64, y: f64, panda_type: PandaType, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Heroo)
        .with(Position(x, y))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(Tile(offset_tile(STANDING, &panda_type)))
        .with(MovingX(Sign::ZERO))
        .with(Velocity(0.0, 0.0))
        .with(LastPush(0.0,0.0))
        .with(Facing(Sign::POSITIVE))
        .with(panda_type)
        .with(Ascending(0.0))
    );
}

pub struct Hero {
    pub moving_x: MovingX,
    pub ascending: Ascending,
    pub x: f64,
    pub y: f64,
    pub dx: f64,
    pub dy: f64,
    pub last_push: (f64, f64),
    pub panda_type: PandaType,
    facing: Sign,
    mesh: ConvexMesh
}

impl Hero {  
    pub fn new(x: f64, y: f64, panda_type: PandaType) -> Self {
        Hero {
            moving_x: MovingX(Sign::ZERO),
            ascending: Ascending(0.0),
            x,
            y,
            dx: 0.0,
            dy: 0.0,
            last_push: (0.0, 0.0),
            facing: Sign::POSITIVE,
            panda_type,
            mesh: ConvexMesh::new(
                vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 
                vec![])
        }
    }

    pub fn mesh(&self) -> ConvexMesh {
        self.mesh.translate(self.x, self.y)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for Hero {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        let (_, last_push_y) = self.last_push;
        let tile = if last_push_y == 0.0 {
            if self.dy > 0.0 { 
                ASCENDING
            } else {
                DESCENDING
            }
        } else if self.dx.abs() < STOPPING_SPEED {
            STANDING
        } else {
            let frame: usize = (self.x / UNITS_PER_FRAME) as usize % RUN_CYCLE.len();
            RUN_CYCLE[frame]
        };
        let (hx, hy) = offset_tile(tile, &self.panda_type);
        let flip_x = self.facing == Sign::NEGATIVE;
        renderer.draw_tile_ex(Sprite { x: hx, y: hy, flip_x, flip_y: false, width: 1, height: 1 }, self.x, self.y);
        Ok(())
    }

    fn event(&mut self, event: &Event, _events: &mut Events) -> Result<(), String> {
        event.apply(|ControllerState { x, jump_held, .. }| {
            if !jump_held {
                self.ascending = Ascending(0.0);
            }
            self.moving_x = MovingX(*x);
        });
        event.apply(|button| jump(self, button));
        event.apply(|dt| update(self, dt));
        Ok(())
    }
}

fn offset_tile((x, y): (i32, i32), panda_type: &PandaType) -> (i32, i32) {
    (x, y + match panda_type {
        PandaType::GiantPanda => PANDA_OFFSET,
        PandaType::RedPanda => RED_PANDA_OFFSET
    })
}

fn update(hero: &mut Hero, dt: &Duration) {
    let dt = dt.as_secs_f64();
    let (last_push_x, last_push_y) = hero.last_push;
    let airborne = last_push_y <= 0.0;
    let x_vel_sign : Sign = hero.dx.sign();
    let MovingX(x_input) = hero.moving_x;
    let Ascending(extrajump) = hero.ascending;

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

    if x_input == Sign::ZERO {
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
    
    if extrajump > 0.0 {
        hero.dy += EXTRA_JUMP * dt;
        hero.ascending = Ascending(extrajump - dt);
    }
    hero.dy -= GRAVITY * dt;

    hero.x += hero.dx * dt;
    hero.y += hero.dy * dt;
}

fn jump(hero: &mut Hero, _event: &ButtonPress) {
    match hero.last_push {
        (_, y) if y > 0.0 => { 
            hero.dy = JUMP_SPEED; 
            hero.ascending = Ascending(EXTRA_JUMP_DURATION);
        },
        (x, _) if x > 0.0 => { 
            hero.dy = WALLJUMP_DY;
            hero.dx = WALLJUMP_DX;
            hero.ascending = Ascending(EXTRA_JUMP_DURATION);

        },
        (x, _) if x < 0.0 => {
            hero.dy = WALLJUMP_DY;
            hero.dx = -WALLJUMP_DX;
            hero.ascending = Ascending(EXTRA_JUMP_DURATION);

        }
        _ => {} 
    }
}