use sdl2::controller::{ Button };
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

use crate::sign::Sign;

#[derive(Clone, Copy)]
pub struct ControllerItem {
    key: Keycode,
    pad: Button,
    pressed: bool,
    fired: bool
}

impl ControllerItem {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(key_pressed), repeat: false, .. } => {
                if key_pressed == &self.key {
                    self.pressed = true;
                    self.fired = true;
                }
            },
            Event::KeyUp { keycode: Some(key_released), repeat: false, .. } => {
                if key_released == &self.key {
                    self.pressed = false;
                }
            },
            Event::ControllerButtonDown { button, .. } => {
                if button == &self.pad {
                    self.pressed = true;
                    self.fired = true;
                }
            }
            Event::ControllerButtonUp { button, .. } => {
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
    left: ControllerItem,
    right: ControllerItem,
    jump: ControllerItem
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, jump: Keycode) -> Self {
        Controller {
            left: ControllerItem{ key: left, pad: Button::DPadLeft, pressed: false, fired: false },
            right: ControllerItem{ key: right, pad: Button::DPadRight, pressed: false, fired: false },
            jump: ControllerItem{ key: jump, pad: Button::A, pressed: false, fired: false }
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        self.left.on_event(event);
        self.right.on_event(event);
        self.jump.on_event(event);
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
}