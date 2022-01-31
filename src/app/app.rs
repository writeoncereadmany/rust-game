use sdl2::event::Event as SdlEvent;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::game_loop::GameLoop;
use crate::graphics::renderer::{ Renderer, Justification };
use crate::game::game::Game;
use crate::fps_counter::FpsCounter;

use super::events::*;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub active_controller: Option<GameController>,
    pub game: Game<'a>,
    pub fps_counter: FpsCounter
}

impl <'a> GameLoop<'a, Renderer<'a>, GEvent> for App<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.clear().unwrap();

        self.game.render(renderer)?;

        renderer.draw_text(self.fps_counter.fps().to_string() + " fps", 2.0, 2.0, Justification::LEFT);      

        renderer.present()?;

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
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
            _ => { }
        }
        self.game.event(event, events)
    }
}