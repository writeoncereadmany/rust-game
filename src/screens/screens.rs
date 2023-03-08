use crate::{game::game::Game, game_loop::GameLoop, graphics::renderer::Renderer, events::{Event, Events}};

pub enum Screen<'a> {
    GameScreen(Game<'a>)
}

impl <'a> GameLoop<'a, Renderer<'a>> for Screen<'a> {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.render(renderer)
        }
    }
 
    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        match self {
            Screen::GameScreen(game) => game.event(event, events)
        }
    }
}