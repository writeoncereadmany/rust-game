use std::time::Duration;

use super::components::*;
use crate::app::events::{Fail, Interaction, SpawnHero};
use crate::controller::{ButtonPress, ControllerState};
use crate::sign::{Sign, Signed};
use component_derive::{Constant, Event, Variable};
use engine::events::*;
use engine::graphics::sprite::Sprite;
use engine::shapes::shape::shape::Shape;
use entity::{entity, Entities};
use crate::entities::hero::PandaType::{GiantPanda, RedPanda};
use crate::entities::pickup::InWater;

const ACCEL: f64 = 60.0;
const REVERSE_ACCEL: f64 = 80.0;
const AIR_ACCEL: f64 = 20.0;
const AIR_SLOWDOWN: f64 = 10.0;
const STOPPING_SPEED: f64 = 1.0;
const VEL_CAP: f64 = 12.0;
const WALLJUMP_DY: f64 = 10.0;
const WALLJUMP_DX: f64 = 10.0;
const WALL_STICK: f64 = 0.1;
const MAX_WALL_FALL_SPEED: f64 = -8.0;
const JUMP_SPEED: f64 = 15.0;
const SPRING_BOUNCE_SPEED: f64 = 45.0;
const EXTRA_JUMP: f64 = 90.0;
const EXTRA_JUMP_DURATION: f64 = 0.2;
const COYOTE_TIME: f64 = 0.1;
const PREJUMP: f64 = 0.1;

const PANDA_OFFSET: i32 = 1;
const RED_PANDA_OFFSET: i32 = 4;

const UNITS_PER_FRAME: f64 = 1.0;
const RUN_CYCLE: [(i32, i32); 4] = [(1, 1), (2, 1), (3, 1), (2, 1)];
const ASCENDING: (i32, i32) = (2, 0);
const DESCENDING: (i32, i32) = (3, 0);
const WALL_DRAGGING: (i32, i32) = (1, 0);
const STANDING: (i32, i32) = (0, 1);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum JumpDirection {
    UP,
    LEFT,
    RIGHT,
    NONE,
}

#[derive(Clone, Copy, Constant)]
pub enum PandaType {
    GiantPanda,
    RedPanda,
}

#[derive(Clone, Constant)]
pub struct Hero;

#[derive(Clone, Variable)]
pub struct IsInWater(pub bool);

#[derive(Clone, Variable)]
pub struct MovingX(pub Sign);

#[derive(Clone, Variable)]
pub struct Ascending(pub f64);

#[derive(Clone, Variable)]
pub struct LastPush(pub f64, pub f64);

#[derive(Clone, Variable)]
pub struct Facing(Sign);

#[derive(Clone, Variable)]
pub struct CoyoteTime(pub JumpDirection, pub f64);

#[derive(Clone, Variable)]
pub struct Prejump(pub f64);

#[derive(Event)]
pub struct Jumped(pub JumpDirection);

pub fn spawn_hero(x: f64, y: f64, panda_type: PandaType, entities: &mut Entities) {
    entities.spawn(entity()
        .with(Hero)
        .with(Gravity)
        .with(Collidable)
        .with(Position(x, y))
        .with(ReferenceMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0)))
        .with(TranslatedMesh(Shape::bbox(0.0, 0.0, 1.0, 1.0).translate(&(x, y))))
        .with(ReferenceContextMesh(Shape::bbox(0.45, 0.45, 0.1, 0.1)))
        .with(TranslatedContextMesh(Shape::bbox(0.45, 0.45, 0.1, 0.1).translate(&(x, y))))
        .with(IsInWater(false))
        .with(offset_sprite(STANDING, &panda_type, false))
        .with(MovingX(Sign::ZERO))
        .with(Velocity(0.0, 0.0))
        .with(LastPush(0.0, 0.0))
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
    event.apply(|&SpawnHero(x, y, panda_type)| spawn_hero(x, y, panda_type, entities));
    event.apply(|&Interaction { hero_id, interaction_type, .. }| { handle_interaction(hero_id, interaction_type, entities) });
    event.apply(|&InWater(hero_id)| { handle_inwater(hero_id, entities)});
}

pub fn clamp_hero(entities: &mut Entities, event: &Event, _events: &mut Events) {
    event.apply(|dt| clamp_to_screen(dt, entities));
}

pub fn clamp_to_screen(_dt: &Duration, entities: &mut Entities) {
    entities.apply(|(Hero, Position(dx, dy))| Position(dx.clamp(0.0, 27.0), dy));
}

pub fn check_fail(entities: &mut Entities, _dt: &Duration, events: &mut Events) {
    entities.apply(|(Hero, Position(_, dy))| if dy < -2.0 { events.fire(Fail) });
}

fn handle_interaction(hero_id: u64, interaction_type: Interacts, entities: &mut Entities) {
    if interaction_type == Interacts::Spring {
        entities.apply_to(&hero_id, |(Hero, Velocity(dx, _dy))| { Velocity(dx, SPRING_BOUNCE_SPEED) });
    }
}

fn handle_inwater(hero_id: u64, entities: &mut Entities) {
    entities.apply_to(&hero_id, |(Hero)| { IsInWater(true) });
}


fn update_hero(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    do_move(entities, dt);
    update_coyote_time(entities, dt);
    check_prejump(entities, dt, events);
    wall_stick(entities, dt);
    uplift(entities, dt);
    clamp(entities, dt);
    facing(entities, dt);
    animate(entities, dt);
    check_fail(entities, dt, events)
}

fn animate(entities: &mut Entities, _dt: &Duration) {
    entities.apply(|(Hero, Position(x, _y), Velocity(dx, dy), Facing(facing), LastPush(px, py), IsInWater(iw), panda_type)| {
        let tile = if py == 0.0 {
            if dy > 0.0 {
                ASCENDING
            } else if px != 0.0 {
                WALL_DRAGGING
            } else {
                DESCENDING
            }
        } else if dx.abs() < STOPPING_SPEED {
            STANDING
        } else {
            let frame: usize = (x / UNITS_PER_FRAME) as usize % RUN_CYCLE.len();
            RUN_CYCLE[frame]
        };
        let mut flip_x = facing == Sign::NEGATIVE;
        if tile == WALL_DRAGGING { flip_x = !flip_x }
        let my_panda_type = if iw { invert(panda_type) } else { panda_type };
        offset_sprite(tile, &my_panda_type, flip_x)
    });
}

fn invert(panda: PandaType) -> PandaType {
    match panda {
        GiantPanda => RedPanda,
        RedPanda => GiantPanda
    }
}

fn control(entities: &mut Entities, &ControllerState { x, jump_held, .. }: &ControllerState) {
    entities.apply(|Ascending(y)| if !jump_held { Ascending(0.0) } else { Ascending(y) });
    entities.apply(|MovingX(_)| MovingX(x));
}

fn do_move(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(Velocity(dx, dy), MovingX(x_input), LastPush(_px, py))| {
        let dt = dt.as_secs_f64();
        let airborne = py <= 0.0;
        if x_input == Sign::ZERO {
            if airborne {
                Velocity(dx - AIR_SLOWDOWN * dt * dx.sign().unit_f64(), dy)
            } else {
                Velocity(dx - REVERSE_ACCEL * dt * dx.sign().unit_f64(), dy)
            }
        } else {
            if airborne {
                Velocity(dx + AIR_ACCEL * dt * x_input.unit_f64(), dy)
            } else if x_input == dx.sign() {
                Velocity(dx + ACCEL * dt * x_input.unit_f64(), dy)
            } else {
                Velocity(dx + REVERSE_ACCEL * dt * x_input.unit_f64(), dy)
            }
        }
    })
}

fn check_prejump(entities: &mut Entities, dt: &Duration, events: &mut Events) {
    entities.apply(|(Prejump(pt), LastPush(px, py))| {
        if pt > 0.0 {
            if py > 0.0 {
                events.fire(Jumped(JumpDirection::UP))
            } else if px > 0.0 {
                events.fire(Jumped(JumpDirection::RIGHT))
            } else if px < 0.0 {
                events.fire(Jumped(JumpDirection::LEFT))
            }
        }
        Prejump(f64::max(pt - dt.as_secs_f64(), 0.0))
    });
}

fn update_coyote_time(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(CoyoteTime(prev_direction, prev_coyote_time), LastPush(px, py))| {
        if py > 0.0 {
            CoyoteTime(JumpDirection::UP, COYOTE_TIME)
        } else if px < 0.0 {
            CoyoteTime(JumpDirection::LEFT, COYOTE_TIME)
        } else if px > 0.0 {
            CoyoteTime(JumpDirection::RIGHT, COYOTE_TIME)
        } else if prev_coyote_time > dt.as_secs_f64()
        {
            CoyoteTime(prev_direction, f64::max(prev_coyote_time - dt.as_secs_f64(), 0.0))
        } else {
            CoyoteTime(JumpDirection::NONE, 0.0)
        }
    })
}

fn wall_stick(entities: &mut Entities, _dt: &Duration) {
    entities.apply(|(Hero, Velocity(dx, dy), MovingX(x_input), LastPush(px, _py))| {
        match (px.sign(), x_input) {
            (Sign::POSITIVE, Sign::NEGATIVE) => Velocity(dx - WALL_STICK, dy.max(MAX_WALL_FALL_SPEED)),
            (Sign::POSITIVE, Sign::ZERO) => Velocity(dx - WALL_STICK, dy),

            (Sign::NEGATIVE, Sign::POSITIVE) => Velocity(dx + WALL_STICK, dy.max(MAX_WALL_FALL_SPEED)),
            (Sign::NEGATIVE, Sign::ZERO) => Velocity(dx + WALL_STICK, dy),

            _otherwise => Velocity(dx, dy)
        }
    })
}

fn uplift(entities: &mut Entities, dt: &Duration) {
    entities.apply(|(Velocity(dx, dy), Ascending(gas))| {
        if gas > 0.0 {
            Velocity(dx, dy + (EXTRA_JUMP * dt.as_secs_f64()))
        } else {
            Velocity(dx, dy)
        }
    });
    entities.apply(|Ascending(gas)| Ascending(f64::max(gas - dt.as_secs_f64(), 0.0)))
}

fn facing(entities: &mut Entities, _dt: &Duration) {
    entities.apply(|(Velocity(dx, _dy), Facing(old_facing))| {
        match dx.sign() {
            Sign::POSITIVE => Facing(Sign::POSITIVE),
            Sign::NEGATIVE => Facing(Sign::NEGATIVE),
            Sign::ZERO => Facing(old_facing)
        }
    })
}

fn clamp(entities: &mut Entities, _dt: &Duration) {
    entities.apply(|(Hero, Velocity(dx, dy), MovingX(x_input))| {
        if x_input == Sign::ZERO && dx.abs() < STOPPING_SPEED {
            Velocity(0.0, dy)
        } else {
            Velocity(dx.clamp(-VEL_CAP, VEL_CAP), dy)
        }
    })
}

fn offset_sprite((x, y): (i32, i32), panda_type: &PandaType, flip_x: bool) -> Sprite {
    Sprite::sprite(x, y + match panda_type {
        PandaType::GiantPanda => PANDA_OFFSET,
        PandaType::RedPanda => RED_PANDA_OFFSET
    }, 1.0, flip_x, false, "Sprites")
}

fn jump(entities: &mut Entities, events: &mut Events, _event: &ButtonPress) {
    entities.apply(|(Hero, CoyoteTime(direction, _ct))| {
        if direction != JumpDirection::NONE {
            events.fire(Jumped(direction));
            Prejump(0.0)
        } else {
            Prejump(PREJUMP)
        }
    })
}

fn on_jump(entities: &mut Entities, Jumped(direction): &Jumped) {
    entities.apply(|(Hero, Velocity(dx, dy))| {
        match direction {
            JumpDirection::UP => Velocity(dx, JUMP_SPEED),
            JumpDirection::RIGHT => Velocity(WALLJUMP_DX, WALLJUMP_DY),
            JumpDirection::LEFT => Velocity(-WALLJUMP_DX, WALLJUMP_DY),
            JumpDirection::NONE => Velocity(dx, dy)
        }
    });
    entities.apply(|Ascending(_)| Ascending(EXTRA_JUMP_DURATION));

    entities.apply(|CoyoteTime(_, _)| CoyoteTime(JumpDirection::NONE, 0.0));
    entities.apply(|Prejump(_)| Prejump(0.0));
}