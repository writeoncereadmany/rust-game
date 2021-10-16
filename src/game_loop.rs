use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub trait Game {

    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, _canvas: &mut Canvas<Window>) -> Result<(), String> {
        Ok(())
    }
 
    fn on_event(&mut self, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<G: Game>(game: &mut G, canvas: &mut Canvas<Window>, events: &mut EventPump) -> Result<(), String> {
    let mut last_frame = Instant::now();
    loop {
        let this_frame = Instant::now();
        for event in events.poll_iter() {
            game.on_event(&event)?;
        }

        game.update(this_frame.duration_since(last_frame))?;

        game.render(canvas)?;

        last_frame = this_frame;
    }
}

