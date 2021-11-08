use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub enum Horizontal {
    LEFT,
    NEUTRAL,
    RIGHT
}

pub enum Vertical {
    UP, 
    NEUTRAL,
    DOWN
}

pub struct Button {
    key: Keycode,
    pressed: bool
}

impl Button {
    fn on_event(&mut self, event: &Event) {
        match event {
            Event::KeyDown { keycode: Some(key_pressed), .. } => {
                if key_pressed == &self.key {
                    self.pressed = true;
                }
            },
            Event::KeyUp { keycode: Some(key_released), .. } => {
                if key_released == &self.key {
                    self.pressed = false;
                }
            },
            _ => ()
        }
    }
}

pub struct Controller {
    left: Button,
    right: Button,
    up: Button, 
    down: Button,
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, up: Keycode, down: Keycode) -> Self {
        Controller {
            left: Button{ key: left, pressed: false},
            right: Button{ key: right, pressed: false},
            up: Button{ key: up, pressed: false},
            down: Button{ key: down, pressed: false},
        }
    }

    pub fn on_event(&mut self, event: &Event) {
        self.left.on_event(event);
        self.right.on_event(event);
        self.up.on_event(event);
        self.down.on_event(event);
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
            (true, false) => -1,
            (false, true) => 1,
            _ => 0
        }   
    }
}