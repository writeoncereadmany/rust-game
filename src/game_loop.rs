use std::time::{Instant, Duration};
use std::fmt::Debug;

use sdl2::EventPump;
use sdl2::event::Event;

use crate::lo_res_renderer::{LoResRenderer};

pub trait Game<T> 
where T: Ord + Debug
{

    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, _renderer: &mut LoResRenderer<T>) -> Result<(), String> {
        Ok(())
    }
 
    fn on_event(&mut self, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<T, G>(game: &mut G, renderer: &mut LoResRenderer<T>, events: &mut EventPump) -> Result<(), String> 
where T: Ord + Debug, G: Game<T>
{
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

