use sdl2::keyboard::Keycode;
use sdl2::event::Event as SdlEvent;

use crate::entities::hero::PandaType;
use crate::graphics::renderer::{Renderer, Text, align};
use crate::game_loop::GameLoop;
use crate::app::events::NewGame;
use crate::app::app::HiScore;

pub struct Scores{ new_hiscore_index: usize, scorer: String, current_letter: String, scores: Vec<HiScore> }

impl Scores {
    pub fn new(latest_score: u32, mut scores: Vec<HiScore>) -> Self {
        let new_hiscore_index = new_hiscore_index(latest_score, &scores);
        scores.insert(new_hiscore_index, HiScore { name: "new".to_string(), score: latest_score });
        scores.truncate(10);

        Scores { new_hiscore_index: new_hiscore_index, scorer: String::new(), current_letter: String::new(), scores }
    }
}

fn new_hiscore_index(latest_score: u32, scores: &Vec<HiScore>) -> usize {
    for (index, HiScore { score, .. }) in scores.iter().enumerate() {
        if &latest_score > score {
            return index;
        }
    }
    return 99;
}

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
        if self.new_hiscore_index <= 10 {
            event.apply(|e| { match e { 
                SdlEvent::KeyDown{ keycode: Some(Keycode::Num1), .. } => events.fire(NewGame(PandaType::GiantPanda)),
                SdlEvent::KeyDown{ keycode: Some(Keycode::Num2), .. } => events.fire(NewGame(PandaType::RedPanda)),
    
                _otherwise => {}
            }});    
        } else {
            event.apply(|e| { match e { 
                SdlEvent::KeyDown{ keycode: Some(Keycode::Num1), .. } => events.fire(NewGame(PandaType::GiantPanda)),
                SdlEvent::KeyDown{ keycode: Some(Keycode::Num2), .. } => events.fire(NewGame(PandaType::RedPanda)),
    
                _otherwise => {}
            }});    

        }
        Ok(())
    }
}