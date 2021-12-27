use sdl2::event::Event as SdlEvent;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::game_loop::*;
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

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>, f64> for App<'a> {

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        renderer.clear(&Layer::FOREGROUND).unwrap();

        self.game.render(renderer)?;

        self.spritefont.render(self.fps_counter.fps().to_string() + " fps", 2, 2, renderer, &Layer::FOREGROUND);      

        renderer.present()?;

        Ok(())
    }

    fn event(&mut self, event: &Event<f64>, events: &mut Events<f64>) -> Result<(), String> {
        match event {
            Event::Sdl(e) => { 
                match e {
                    SdlEvent::Quit {..} => return Err("Escape pressed: ending game".into()),
                    SdlEvent::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
                    SdlEvent::ControllerDeviceAdded{ which, .. } => { 
                        self.active_controller = self.game_controller_subsystem.open(*which).ok();
                    },
                    _ => {}
                }
            },
            Event::Time(_) => { self.fps_counter.on_frame(); },
            Event::Game(_) => { }
        }
        self.game.event(event, events)
    }
}