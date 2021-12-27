use std::collections::VecDeque;
use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event as SdlEvent;

pub enum Event<E> {
    Sdl(SdlEvent),
    Time(Duration),
    Game(E),
    Cleanup,
}

pub struct Events<E> {
    events: VecDeque<Event<E>>
}

impl <E> Events<E> {

    fn new() -> Self {
        Events{ events: VecDeque::new() }
    }

    pub fn fire(&mut self, event: Event<E>) {
        self.events.push_back(event);
    }
}

pub trait GameLoop<'a, R, E>
{
    fn render(&self, _renderer: &mut R) -> Result<(), String> {
        Ok(())
    }
 
    fn event(&mut self, _event: &Event<E>, _events: &mut Events<E>) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<'a, R, G, E>(game: &'a mut G, renderer: &mut R, sdl_events: &mut EventPump) -> Result<(), String> 
where G: GameLoop<'a, R, E>
{
    let mut last_frame = Instant::now();
    let mut events: Events<E> = Events::new();
    loop {
        let this_frame = Instant::now();
        for event in sdl_events.poll_iter() {
            events.fire(Event::Sdl(event));
        }

        events.fire(Event::Time(this_frame.duration_since(last_frame)));

        let mut event = events.events.pop_front();
        loop {
            match event {
                None => { break; }
                Some(e) => { 
                    game.event(&e, &mut events)?;
                    event = events.events.pop_front();
                }
            }
        }

        game.event(&Event::Cleanup, &mut events)?;

        game.render(renderer)?;

        last_frame = this_frame;
    }
}

