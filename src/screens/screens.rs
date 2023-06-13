use engine::graphics::renderer::Renderer;

use crate::events::Events;
use crate::events::Event;
use crate::game_loop::GameLoop;
use crate::game::game::Game;

use super::hi_score::Scores;
use super::title::Title;

pub enum Screen<'a> {
    GameScreen(Game<'a>),
    TitleScreen(Title),
    HiScoreScreen(Scores)
}

impl <'a> GameLoop<'a, Renderer<'a>> for Screen<'a> {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.render(renderer),
            Screen::TitleScreen(title) => title.render(renderer),
            Screen::HiScoreScreen(scores) => scores.render(renderer)
        }
    }
 
    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.event(event, events),
            Screen::TitleScreen(title) => title.event(event, events),
            Screen::HiScoreScreen(scores) => scores.event(event, events)
        }
    }
}