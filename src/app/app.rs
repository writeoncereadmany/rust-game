use std::time::Duration;

use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::game_loop::GameEvents;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::graphics::text_renderer::SpriteFont;
use crate::world::world::World;
use crate::fps_counter::FpsCounter;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub active_controller: Option<GameController>,
    pub world: World<'a>,
    pub fps_counter: FpsCounter,
    pub spritefont: &'a SpriteFont<'a>,
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for App<'a> {

    fn update(&mut self, dt: Duration) -> Result<(), String> {
        self.world.update(dt)
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.fps_counter.on_frame();
        renderer.clear(&Layer::FOREGROUND).unwrap();

        self.world.render(renderer)?;

        self.spritefont.render(self.fps_counter.fps().to_string() + " fps", 2, 2, renderer, &Layer::FOREGROUND);      

        renderer.present()
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        match event {
            Event::Quit {..} => return Err("Escape pressed: ending game".into()),
            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
            Event::ControllerDeviceAdded{ which, .. } => { 
                self.active_controller = self.game_controller_subsystem.open(*which).ok(); 
            }
            _ => {}
        }

        self.world.on_event(event)
    }
}