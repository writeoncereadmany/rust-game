use sdl2::controller::Button;
use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;

use component_derive::Event;
use engine::events::{Event, EventTrait, Events};
use engine::game_loop::CascadeInputs;

use crate::sign::Sign;

#[derive(Event)]
pub struct ControllerState {
    pub id: u64,
    pub x: Sign,
    pub jump_held: bool,
}

#[derive(Clone, Copy, Event)]
pub struct ButtonPress(u64);

#[derive(Clone, Copy)]
pub struct ControllerItem {
    key: Keycode,
    pad: Button,
    pressed: bool,
    on_press: Option<ButtonPress>,
}

impl ControllerItem {
    fn on_event(&mut self, event: &SdlEvent, events: &mut Events) {
        match event {
            SdlEvent::KeyDown { keycode: Some(key_pressed), repeat: false, .. } => {
                if key_pressed == &self.key {
                    self.pressed = true;
                    if let Some(x) = self.on_press {
                        events.fire(x);
                    }
                }
            }
            SdlEvent::KeyUp { keycode: Some(key_released), repeat: false, .. } => {
                if key_released == &self.key {
                    self.pressed = false;
                }
            }
            SdlEvent::ControllerButtonDown { button, .. } => {
                if button == &self.pad {
                    self.pressed = true;
                    if let Some(x) = self.on_press {
                        events.fire(x);
                    }
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
}

#[derive(Clone, Copy)]
pub struct Controller {
    id: u64,
    left: ControllerItem,
    right: ControllerItem,
    jump: ControllerItem,
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, jump: Keycode) -> Self {
        Controller {
            id: 1,
            left: ControllerItem { key: left, pad: Button::DPadLeft, pressed: false, on_press: None },
            right: ControllerItem { key: right, pad: Button::DPadRight, pressed: false, on_press: None },
            jump: ControllerItem { key: jump, pad: Button::A, pressed: false, on_press: Some(ButtonPress(1)) },
        }
    }

    pub fn on_event(&mut self, event: &Event, events: &mut Events) {
        event.apply(|e| self.left.on_event(e, events));
        event.apply(|e| self.right.on_event(e, events));
        event.apply(|e| self.jump.on_event(e, events));
        event.apply(|e| self.fire_new_state(e, events));
    }

    pub fn x(&self) -> Sign {
        match (self.left.pressed, self.right.pressed) {
            (true, false) => Sign::NEGATIVE,
            (false, true) => Sign::POSITIVE,
            _ => Sign::ZERO
        }
    }

    pub fn jump_held(&self) -> bool {
        self.jump.pressed
    }

    pub fn fire_new_state(&self, _cascade: &CascadeInputs, events: &mut Events) {
        events.fire(ControllerState {
            id: self.id,
            x: self.x(),
            jump_held: self.jump_held(),
        });
    }
}