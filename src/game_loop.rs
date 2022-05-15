use std::time::Instant;

use sdl2::EventPump;

use component_derive::Event;

use crate::events::*;

#[derive(Event)]
pub struct Cleanup;

#[derive(Event)]
pub struct CascadeInputs;

pub trait GameLoop<'a, R>
{
    fn render(&self, _renderer: &mut R) -> Result<(), String> {
        Ok(())
    }
 
    fn event(&mut self, _event: &Event, _events: &mut Events) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<'a, R, G>(mut game: G, renderer: &mut R, sdl_events: &mut EventPump, updates_per_frame: u32, mut events: Events) -> Result<(), String> 
where G: GameLoop<'a, R>
{
    let mut last_frame = Instant::now();
    let cleanup = Event::new(Cleanup);
    loop {
        let this_frame = Instant::now();
        for event in sdl_events.poll_iter() {
            events.fire(event);
        }
        events.fire(CascadeInputs);

        while let Some(event) = events.pop() {
            game.event(&event, &mut events)?;
        }

        for _ in 0..updates_per_frame {
            let update_duration = this_frame.duration_since(last_frame).div_f64(updates_per_frame as f64);
            events.elapse(update_duration);
            events.fire(update_duration);

            while let Some(event) = events.pop() {
                game.event(&event, &mut events)?;
            }
        }

        game.event(&cleanup, &mut events)?;

        game.render(renderer)?;

        last_frame = this_frame;
    }
}

