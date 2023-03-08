use crate::{game::game::Game, game_loop::GameLoop, graphics::renderer::Renderer, events::{Event, Events}};

use super::title::Title;

pub enum Screen<'a> {
    GameScreen(Game<'a>),
    TitleScreen(Title)
}

impl <'a> GameLoop<'a, Renderer<'a>> for Screen<'a> {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.render(renderer),
            Screen::TitleScreen(title) => title.render(renderer)
        }
    }
 
    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.event(event, events),
            Screen::TitleScreen(title) => title.event(event, events)
        }
    }
}