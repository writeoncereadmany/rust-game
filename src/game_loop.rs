use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event;

use crate::lo_res_renderer::LoResRenderer;

pub trait Game {

    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, _renderer: &mut LoResRenderer) -> Result<(), String> {
        Ok(())
    }
 
    fn on_event(&mut self, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<G: Game>(game: &mut G, renderer: &mut LoResRenderer, events: &mut EventPump) -> Result<(), String> {
    let mut last_frame = Instant::now();
    loop {
        let this_frame = Instant::now();
        for event in events.poll_iter() {
            game.on_event(&event)?;
        }

        game.update(this_frame.duration_since(last_frame))?;

        game.render(renderer)?;

        last_frame = this_frame;
    }
}

