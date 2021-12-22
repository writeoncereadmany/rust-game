use sdl2::event::{Event};
use sdl2::keyboard::Keycode;

pub struct Button {
    key: Keycode,
    pressed: bool,
    fired: bool
}

impl Button {
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
    left: Button,
    right: Button,
    up: Button, 
    down: Button,
    jump: Button
}

impl Controller {
    pub fn new(left: Keycode, right: Keycode, up: Keycode, down: Keycode, jump: Keycode) -> Self {
        Controller {
            left: Button{ key: left, pressed: false, fired: false },
            right: Button{ key: right, pressed: false, fired: false },
            up: Button{ key: up, pressed: false, fired: false },
            down: Button{ key: down, pressed: false, fired: false },
            jump: Button{ key: jump, pressed: false, fired: false }
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