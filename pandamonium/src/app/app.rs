use sdl2::VideoSubsystem;
use sdl2::event::Event as SdlEvent;
use sdl2::audio::AudioDevice;
use sdl2::GameControllerSubsystem;
use sdl2::keyboard::Keycode;
use sdl2::controller::GameController;

use engine::graphics::renderer::Renderer;
use engine::audio::audio::*;
use engine::events::*;
use engine::game_loop::*;

use crate::controller::Controller;
use crate::game::game::Game;
use crate::screens::hi_score::Scores;
use crate::screens::screens::Screen;
use crate::screens::title::Title;

use super::assets::Assets;
use super::events::UpdateHiScores;
use super::events::{ ClearAudio, GameOver, NewGame, ShowHighScores, ShowTitleScreen };

#[derive(Clone)]
pub struct HiScore {
    pub name: String,
    pub score: u32
}

pub struct App<'a> {
    pub video_subsystem: VideoSubsystem,
    pub game_controller_subsystem: GameControllerSubsystem,
    // pub audio_device: AudioDevice<AudioPlayer>,
    pub active_controller: Option<GameController>,
    pub controller: Controller,
    pub assets: &'a Assets<'a>,
    pub screen: Screen<'a>,
    pub scores: Vec<HiScore>
}

impl <'a> App<'a> {
    pub fn starting_scores() -> Vec<HiScore> {
        vec![
            HiScore { name: "Anne".to_string(), score: 1000},
            HiScore { name: "Bill".to_string(), score: 900},
            HiScore { name: "Carl".to_string(), score: 800},
            HiScore { name: "Dina".to_string(), score: 700},
            HiScore { name: "Elsa".to_string(), score: 600},
            HiScore { name: "Fred".to_string(), score: 500},
            HiScore { name: "Gwen".to_string(), score: 400},
            HiScore { name: "Hank".to_string(), score: 300},
            HiScore { name: "Iris".to_string(), score: 200},
            HiScore { name: "Jake".to_string(), score: 100},
        ]
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for App<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.clear().unwrap();

        self.screen.render(renderer)?;

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
        // event.apply(|ClearAudio()| { self.audio_device.lock().clear(); } );
        // event.apply(|tune| play_tune(&mut self.audio_device, tune));
        event.apply(|NewGame(panda_type)| { self.screen = Screen::GameScreen(Game::new(*panda_type, self.assets, events))});
        event.apply(|GameOver(score)| { self.screen = Screen::HiScoreScreen(Scores::new(*score, self.scores.clone()))});
        event.apply(|ShowHighScores()| { self.screen = Screen::HiScoreScreen(Scores::new(0, self.scores.clone()))});
        event.apply(|ShowTitleScreen()| { self.screen = Screen::TitleScreen(Title)});
        event.apply(|UpdateHiScores(scores)| { self.scores = scores.clone() });

        self.controller.on_event(event, events);
        self.screen.event(event, events)
    }
}