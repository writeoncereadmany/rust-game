use sdl2::controller::{ Button };
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;

use component_derive::Event;
use crate::events::{Event, Events, EventTrait};
use crate::game_loop::CascadeInputs;

use crate::sign::Sign;

#[derive(Event)]
pub struct ControllerState {
    pub id: u64,
    pub x: Sign,
    pub jump_held: bool,
}

#[derive(Clone, Copy)]
pub struct ControllerItem {
    key: Keycode,
    pad: Button,
    pressed: bool,
    fired: bool
}

impl ControllerItem {
    fn on_event(&mut self, event: &SdlEvent) {
        match event {
            SdlEvent::KeyDown { keycode: Some(key_pressed), repeat: false, .. } => {
                if key_pressed == &self.key {
                    self.pressed = true;
                    self.fired = true;
                }
            },
            SdlEvent::KeyUp { keycode: Some(key_released), repeat: false, .. } => {
                if key_released == &self.key {
                    self.pressed = false;
                }
            },
            SdlEvent::ControllerButtonDown { button, .. } => {
                if button == &self.pad {
                    self.pressed = true;
                    self.fired = true;
                }
            }
            SdlEvent::ControllerButtonUp { button, .. } => {
                if button == &self.pad {
                    self.pressed = false;
                }
            }
            _ => ()
        }
    }

    fn fired(&mut self) -> bool {
        if self.fired {
            self.fired = false;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Copy)]
pub struct Controller {
    id: u64,
    left: ControllerItem,
    right: ControllerItem,
    jump: ControllerItem
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, jump: Keycode) -> Self {
        Controller {
            id: 1,
            left: ControllerItem{ key: left, pad: Button::DPadLeft, pressed: false, fired: false },
            right: ControllerItem{ key: right, pad: Button::DPadRight, pressed: false, fired: false },
            jump: ControllerItem{ key: jump, pad: Button::A, pressed: false, fired: false }
        }
    }

    pub fn on_event(&mut self, event: &Event, mut events: &mut Events) {
        event.apply(|e| self.left.on_event(e));
        event.apply(|e| self.right.on_event(e));
        event.apply(|e| self.jump.on_event(e));
        event.apply(|e| self.fire_new_state(e, &mut events));
    }

    pub fn x(&self) -> Sign {
        match (self.left.pressed, self.right.pressed) {
            (true, false) => Sign::NEGATIVE,
            (false, true) => Sign::POSITIVE,
            _ => Sign::ZERO
        }
    }

    pub fn jump_pressed(&mut self) -> bool {
        self.jump.fired()
    }

    pub fn jump_held(&self) -> bool {
        self.jump.pressed
    }

    pub fn fire_new_state(&self, _cascade: &CascadeInputs, events: &mut Events) {
        events.fire(ControllerState {
            id: self.id,
            x: self.x(),
            jump_held: self.jump_held()
        });
    }
}