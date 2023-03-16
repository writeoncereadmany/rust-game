use sdl2::keyboard::Keycode;
use sdl2::event::Event as SdlEvent;


use crate::entities::hero::PandaType;
use crate::graphics::renderer::{Renderer, Text, align};
use crate::game_loop::GameLoop;
use crate::app::events::{NewGame, ShowHighScores};

pub struct Title;

impl <'a> GameLoop<'a, Renderer<'a>> for Title {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_text(&Text { text: "PANDAMONIUM".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 10.0);
        renderer.draw_text(&Text { text: "1: play as Blue".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 5.0);
        renderer.draw_text(&Text { text: "2: play as Redd".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 4.0);
        renderer.draw_text(&Text { text: "H: see hiscores".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 3.0);


        Ok(())
    }

    fn event(&mut self, event: &crate::events::Event, events: &mut crate::events::Events) -> Result<(), String> {
        event.apply(|e| { match e { 
            SdlEvent::KeyDown{ keycode: Some(Keycode::Num1), .. } => events.fire(NewGame(PandaType::GiantPanda)),
            SdlEvent::KeyDown{ keycode: Some(Keycode::Num2), .. } => events.fire(NewGame(PandaType::RedPanda)),
            SdlEvent::KeyDown{ keycode: Some(Keycode::H), .. } => events.fire(ShowHighScores()),


            _otherwise => {}
        }
        });
        Ok(())
    }
}