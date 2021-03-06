use std::time::Duration;

use component_derive::{ Event, Constant, Variable };
use entity::{ entity, Entities, Component, Variable };

use crate::controller::{ ButtonPress, ControllerState };
use crate::events::*;
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
const COYOTE_TIME: f64 = 0.1;
const PREJUMP: f64 = 0.1;

const PANDA_OFFSET: i32 = 1;
const RED_PANDA_OFFSET: i32 = 4;

const UNITS_PER_FRAME: f64 = 1.0;
const RUN_CYCLE : [(i32, i32); 4] = [(1, 1), (2, 1), (3, 1), (2, 1)];
const ASCENDING : (i32, i32) = (2, 0);
const DESCENDING : (i32, i32) = (3, 0);
const STANDING: (i32, i32) = (0, 1);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum JumpDirection {
    UP,
    LEFT,
    RIGHT,
    NONE
}

#[derive(Clone, Copy, Constant)]
pub enum PandaType {
    GiantPanda,
    RedPanda
}

#[derive(Constant)]
pub struct Hero;

#[derive(Variable)]
pub struct MovingX(pub Sign);

#[derive(Variable)]
pub struct Ascending(pub f64);

#[derive(Variable)]
pub struct LastPush(pub f64, pub f64);

#[derive(Variable)]
pub struct Facing(Sign);

#[derive(Variable)]
pub struct CoyoteTime(pub JumpDirection, pub f64);

#[derive(Variable)]
pub struct Prejump(pub f64);

#[derive(Event)]
pub struct Jumped(pub JumpDirection);

pub fn spawn_hero(x: f64, y: f64, panda_type: PandaType, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Hero)
        .with(Position(x, y))
        .with(ReferenceMesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![])))
        .with(Mesh(ConvexMesh::new(vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], vec![]).translate(x, y)))
        .with(offset_sprite(STANDING, &panda_type, false))
        .with(MovingX(Sign::ZERO))
        .with(Velocity(0.0, 0.0))
        .with(LastPush(0.0,0.0))
        .with(Facing(Sign::POSITIVE))
        .with(panda_type)
        .with(CoyoteTime(JumpDirection::NONE, 0.0))
        .with(Prejump(0.0))
        .with(Ascending(0.0))
    );
}

pub fn hero_events(entities: &mut Entities, event: &Event, events: &mut Events) {
    event.apply(|controller| control(entities, controller));
    event.apply(|buttonpress| jump(entities, events, buttonpress));
    event.apply(|jump| on_jump(entities, jump));
    event.apply(|dt| update_hero(entities, dt, events));
}

fn update_hero(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    do_move(entities, dt);
    update_coyote_time(entities, dt);
    check_prejump(entities, dt, events);
    wall_stick(entities, dt);
    gravity(entities, dt);
    uplift(entities, dt);
    clamp(entities, dt);
    integrate(entities, dt);
    translate(entities, dt);
    update_box(entities, dt);
    facing(entities, dt);
    animate(entities, dt);
}

fn animate(entities: &mut Entities, _dt: &Duration) {
    entities.apply_6(| &Hero, &Position(x, _y), &Velocity(dx, dy), &Facing(facing), &LastPush(_px, py), panda_type : &PandaType | {
        let tile = if py == 0.0 {
            if dy > 0.0 { 
                ASCENDING
            } else {
                DESCENDING
            }
        } else if dx.abs() < STOPPING_SPEED {
            STANDING
        } else {
            let frame: usize = (x / UNITS_PER_FRAME) as usize % RUN_CYCLE.len();
            RUN_CYCLE[frame]
        };
        let flip_x = facing == Sign::NEGATIVE;
        offset_sprite(tile, panda_type, flip_x)
    });
}

fn control(entities: &mut Entities, &ControllerState { x, jump_held, .. }: &ControllerState ) {
    entities.apply(|&Ascending(y)| if !jump_held { Ascending(0.0) } else { Ascending(y) });
    entities.apply(|&MovingX(_)| MovingX(x) );
}

fn do_move(entities: &mut Entities, dt: &Duration) {
    entities.apply_3(|&Velocity(dx, dy), &MovingX(x_input), &LastPush(_px, py)| {
        let dt = dt.as_secs_f64();
        let airborne = py <= 0.0;
        if x_input == Sign::ZERO {
            if airborne {
                Velocity(dx - AIR_SLOWDOWN * dt * dx.sign().unit_f64(), dy)
            }
            else {
                Velocity(dx - REVERSE_ACCEL * dt * dx.sign().unit_f64(), dy)
            }
        } else {
            if airborne {
                Velocity(dx + AIR_ACCEL * dt * x_input.unit_f64(), dy)
            }
            else if x_input == dx.sign() {
                Velocity(dx + ACCEL * dt * x_input.unit_f64(), dy)
            } else {
                Velocity(dx + REVERSE_ACCEL * dt * x_input.unit_f64(), dy)
            }
        }
    })
}

fn check_prejump(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    entities.apply_2(|&Prejump(pt), &LastPush(px, py)| {
        if pt > 0.0 {
            if py > 0.0 {
                events.fire(Jumped(JumpDirection::UP))
            } 
            else if px > 0.0 {
                events.fire(Jumped(JumpDirection::RIGHT))
            }
            else if px < 0.0 {
                events.fire(Jumped(JumpDirection::LEFT))
            }
        }
        Prejump(f64::max(pt - dt.as_secs_f64(), 0.0))
    });
}

fn update_coyote_time(entities: &mut Entities, dt: &Duration) {
    entities.apply_2(|&CoyoteTime(prev_direction, prev_coyote_time), &LastPush(px, py)| {
        if py > 0.0 {
            CoyoteTime(JumpDirection::UP, COYOTE_TIME)
        }
        else if px < 0.0 {
            CoyoteTime(JumpDirection::LEFT, COYOTE_TIME)
        }
        else if px > 0.0 {
            CoyoteTime(JumpDirection::RIGHT, COYOTE_TIME)
        }
        else if prev_coyote_time > dt.as_secs_f64()
        {
            CoyoteTime(prev_direction, f64::max(prev_coyote_time - dt.as_secs_f64(), 0.0))
        }
        else 
        {
            CoyoteTime(JumpDirection::NONE, 0.0)
        }
    })
}

fn gravity(entities: &mut Entities, dt: &Duration) {
    entities.apply(|&Velocity(dx, dy)| Velocity(dx, dy - GRAVITY * dt.as_secs_f64()))
}

fn integrate(entities: &mut Entities, dt: &Duration) {
    entities.apply(|&Velocity(dx, dy)| Translation(dx * dt.as_secs_f64(), dy * dt.as_secs_f64()));
}

fn translate(entities: &mut Entities, _dt: &Duration) {
    entities.apply_2(|&Translation(tx, ty), &Position(x, y)| Position(x + tx, y + ty));
}

fn wall_stick(entities: &mut Entities, _dt: &Duration) {
    entities.apply_2(|&Velocity(dx, dy), &LastPush(px, _py)| {
        match px.sign() {
            Sign::POSITIVE => Velocity(dx -WALL_STICK, dy),
            Sign::NEGATIVE => Velocity(dx + WALL_STICK, dy),
            Sign::ZERO => Velocity(dx, dy)
        }
    })
}

fn uplift(entities: &mut Entities, dt: &Duration) {
    entities.apply_2(|&Velocity(dx, dy), &Ascending(gas)| {
        if gas > 0.0 {
            Velocity(dx, dy + (EXTRA_JUMP * dt.as_secs_f64()))
        } else {
            Velocity(dx, dy)
        }
    });
    entities.apply(|&Ascending(gas)| Ascending(f64::max(gas - dt.as_secs_f64(), 0.0)))
}

fn facing(entities: &mut Entities, _dt: &Duration) {
    entities.apply_2(|&Velocity(dx, _dy), &Facing(old_facing)| {
        match dx.sign() {
            Sign::POSITIVE => Facing(Sign::POSITIVE),
            Sign::NEGATIVE => Facing(Sign::NEGATIVE),
            Sign::ZERO => Facing(old_facing)
        }
    })
}

fn clamp(entities: &mut Entities, _dt: &Duration) {
    entities.apply_2(|&Velocity(dx, dy), &MovingX(x_input)| {
        if x_input == Sign::ZERO && dx.abs() < STOPPING_SPEED {
            Velocity(0.0, dy)
        } else {
            Velocity(dx.clamp(-VEL_CAP, VEL_CAP), dy)
        }
    })
}

fn update_box(entities: &mut Entities, _dt: &Duration) {
    entities.apply_2(|&Position(x, y), ReferenceMesh(mesh)| Mesh(mesh.translate(x, y)))
}


fn offset_sprite((x, y): (i32, i32), panda_type: &PandaType, flip_x: bool) -> Sprite {
    Sprite::sprite(x, y + match panda_type {
        PandaType::GiantPanda => PANDA_OFFSET,
        PandaType::RedPanda => RED_PANDA_OFFSET
    }, 1.0, flip_x, false)
}

fn jump(entities: &mut Entities, events: &mut Events, _event: &ButtonPress) {
    entities.apply(|&CoyoteTime(direction, _ct)| {
        if direction != JumpDirection::NONE {
            events.fire(Jumped(direction));
            Prejump(0.0)
        } else {
            Prejump(PREJUMP)
        }
    })
}

fn on_jump(entities: &mut Entities, Jumped(direction): &Jumped) {
    entities.apply(|&Velocity(dx, dy)| {
        match direction {
            JumpDirection::UP => Velocity(dx, JUMP_SPEED),
            JumpDirection::RIGHT => Velocity(WALLJUMP_DX, WALLJUMP_DY),
            JumpDirection::LEFT => Velocity(-WALLJUMP_DX, WALLJUMP_DY),
            JumpDirection::NONE => Velocity(dx, dy)
        }
    });
    entities.apply(|&Ascending(_)| Ascending(EXTRA_JUMP_DURATION));

    entities.apply(|&CoyoteTime(_, _)| CoyoteTime(JumpDirection::NONE, 0.0));
    entities.apply(|&Prejump(_)| Prejump(0.0));
}