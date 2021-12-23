use sdl2::controller::{ Button };
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

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

pub struct Controller {
    left: ControllerItem,
    right: ControllerItem,
    up: ControllerItem, 
    down: ControllerItem,
    jump: ControllerItem
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, up: Keycode, down: Keycode, jump: Keycode) -> Self {
        Controller {
            left: ControllerItem{ key: left, pad: Button::DPadLeft, pressed: false, fired: false },
            right: ControllerItem{ key: right, pad: Button::DPadRight, pressed: false, fired: false },
            up: ControllerItem{ key: up, pad: Button::DPadUp, pressed: false, fired: false },
            down: ControllerItem{ key: down, pad: Button::DPadDown, pressed: false, fired: false },
            jump: ControllerItem{ key: jump, pad: Button::A, pressed: false, fired: false }
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        self.left.on_event(event);
        self.right.on_event(event);
        self.up.on_event(event);
        self.down.on_event(event);
        self.jump.on_event(event);
    }

    pub fn x(&self) -> i32 {
        match (self.left.pressed, self.right.pressed) {
            (true, false) => -1,
            (false, true) => 1,
            _ => 0
        }
    }

    pub fn y(&self) -> i32 {
        match (self.up.pressed, self.down.pressed) {
            (true, false) => 1,
            (false, true) => -1,
            _ => 0
        }   
    }

    pub fn jump(&mut self) -> bool {
        self.jump.fired()
    }
}