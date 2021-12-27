use std::time::Duration;

use sdl2::event::Event;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::game_loop::GameLoop;
use crate::graphics::renderer::Renderer;
use crate::graphics::lo_res_renderer::{Layer, LoResRenderer};
use crate::graphics::text_renderer::SpriteFont;
use crate::game::game::Game;
use crate::fps_counter::FpsCounter;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub active_controller: Option<GameController>,
    pub game: Game<'a>,
    pub fps_counter: FpsCounter,
    pub spritefont: &'a SpriteFont<'a>,
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>> for App<'a> {

    fn update(&mut self, dt: Duration) -> Result<(), String> {
        self.game.update(dt)
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.clear(&Layer::FOREGROUND).unwrap();

        self.game.render(renderer)?;

        self.spritefont.render(self.fps_counter.fps().to_string() + " fps", 2, 2, renderer, &Layer::FOREGROUND);      

        renderer.present()?;
        self.fps_counter.on_frame();
        Ok(())
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

        self.game.on_event(event)
    }
}