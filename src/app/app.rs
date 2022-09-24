use std::time::Duration;

use sdl2::event::Event as SdlEvent;
use sdl2::audio::AudioDevice;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use crate::audio::audio::*;
use crate::game_loop::*;
use crate::events::*;
use crate::graphics::renderer::{ Renderer };
use crate::game::game::Game;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub audio_device: AudioDevice<AudioPlayer>,
    pub active_controller: Option<GameController>,
    pub game: Game<'a>
}

impl <'a> GameLoop<'a, Renderer<'a>> for App<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.clear().unwrap();

        self.game.render(renderer)?;

        renderer.present()?;

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        if let Some(e) = event.unwrap() {
            match e {
                SdlEvent::Quit {..} => return Err("Escape pressed: ending game".into()),
                SdlEvent::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
                SdlEvent::ControllerDeviceAdded{ which, .. } => { 
                    self.active_controller = self.game_controller_subsystem.open(*which).ok();
                },
                _ => {}
            }
        }
        event.apply(|tune| play_tune(&mut self.audio_device, tune));


        self.game.event(event, events)
    }
}