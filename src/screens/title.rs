use sdl2::keyboard::Keycode;
use sdl2::event::Event as SdlEvent;


use crate::graphics::renderer::{Renderer, Text, align};
use crate::game_loop::GameLoop;
use crate::app::events::NewGame;

pub struct Title;

impl <'a> GameLoop<'a, Renderer<'a>> for Title {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        renderer.draw_text(&Text { text: "PANDAMONIUM".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 10.0);
        renderer.draw_text(&Text { text: "Press SPACE to play".to_string(), justification:  align::CENTER | align::MIDDLE }, 13.0, 5.0);

        Ok(())
    }

    fn event(&mut self, event: &crate::events::Event, events: &mut crate::events::Events) -> Result<(), String> {
        event.apply(|e| { match e { 
            SdlEvent::KeyDown{ keycode: Some(Keycode::Space), .. } => events.fire(NewGame()),
            _otherwise => {}
        }
        });
        Ok(())
    }
}