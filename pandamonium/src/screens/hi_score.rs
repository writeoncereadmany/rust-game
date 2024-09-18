use sdl2::event::Event as SdlEvent;
use sdl2::keyboard::Keycode;

use engine::events::{Event, Events};
use engine::game_loop::GameLoop;
use engine::graphics::renderer::{align, Renderer, Text};

use crate::app::app::HiScore;
use crate::app::events::{NewGame, ShowTitleScreen, UpdateHiScores};
use crate::entities::hero::PandaType;

pub struct Scores {
    new_hiscore_index: usize,
    scores: Vec<HiScore>,
}

impl Scores {
    pub fn new(latest_score: u32, mut scores: Vec<HiScore>) -> Self {
        let new_hiscore_index = new_hiscore_index(latest_score, &scores);

        if new_hiscore_index < 10 {
            scores.insert(new_hiscore_index, HiScore { name: "".to_string(), score: latest_score });
            scores.truncate(10);
        }

        Scores { new_hiscore_index, scores }
    }

    fn update_name(&mut self, text: &String)
    {
        let current_name = &mut self.scores[self.new_hiscore_index].name;
        current_name.push_str(text);
        current_name.truncate(4);
    }

    fn trim_name(&mut self)
    {
        let current_name = &mut self.scores[self.new_hiscore_index].name;
        current_name.truncate(current_name.len() - 1);
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

impl<'a> GameLoop<'a, Renderer<'a>> for Scores {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_text(&Text { text: "HIGH SCORES".to_string(), justification: align::CENTER | align::MIDDLE }, 13.0, 14.0);

        for (index, HiScore { name, score }) in self.scores.iter().enumerate() {
            if index < 10 {
                renderer.draw_text(&Text { text: name.to_string(), justification: align::RIGHT | align::MIDDLE }, 13.0, 12.0 - index as f64);
                renderer.draw_text(&Text { text: score.to_string(), justification: align::LEFT | align::MIDDLE }, 14.0, 12.0 - index as f64);
            }
        }
        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        if self.new_hiscore_index > 10 {
            event.apply(|e| {
                match e {
                    SdlEvent::KeyDown { keycode: Some(Keycode::Num1), .. } => events.fire(NewGame(PandaType::GiantPanda)),
                    SdlEvent::KeyDown { keycode: Some(Keycode::Num2), .. } => events.fire(NewGame(PandaType::RedPanda)),

                    _otherwise => {}
                }
            });
        } else {
            event.apply(|e| {
                match e {
                    SdlEvent::TextInput { text, .. } => self.update_name(text),
                    SdlEvent::KeyDown { keycode: Some(Keycode::Backspace), .. } => self.trim_name(),
                    SdlEvent::KeyDown { keycode: Some(Keycode::Return), .. } => {
                        events.fire(UpdateHiScores(self.scores.clone()));
                        events.fire(ShowTitleScreen());
                    }

                    _otherwise => {}
                }
            });
        }
        Ok(())
    }
}