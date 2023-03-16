use sdl2::keyboard::Keycode;
use sdl2::event::Event as SdlEvent;

use crate::entities::hero::PandaType;
use crate::graphics::renderer::{Renderer, Text, align};
use crate::game_loop::GameLoop;
use crate::app::events::NewGame;
use crate::app::app::HiScore;

pub struct Scores{ pub scores: Vec<HiScore> }

impl <'a> GameLoop<'a, Renderer<'a>> for Scores {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_text(&Text { text: "HIGH SCORES".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 14.0);

        for (index, HiScore { name, score }) in self.scores.iter().enumerate() {
            if index < 10 {
                renderer.draw_text(&Text { text: name.to_string(), justification:  align::RIGHT | align::MIDDLE }, 13.0, 12.0 - index as f64);
                renderer.draw_text(&Text { text: score.to_string(), justification:  align::LEFT | align::MIDDLE }, 14.0, 12.0 - index as f64);

            }
        }
        Ok(())
    }

    fn event(&mut self, event: &crate::events::Event, events: &mut crate::events::Events) -> Result<(), String> {
        event.apply(|e| { match e { 
            SdlEvent::KeyDown{ keycode: Some(Keycode::Num1), .. } => events.fire(NewGame(PandaType::GiantPanda)),
            SdlEvent::KeyDown{ keycode: Some(Keycode::Num2), .. } => events.fire(NewGame(PandaType::RedPanda)),

            _otherwise => {}
        }
        });
        Ok(())
    }
}