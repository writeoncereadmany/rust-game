use std::collections::VecDeque;
use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event as SdlEvent;

pub enum Event<E> {
    Sdl(SdlEvent),
    Time(Duration),
    Game(E)
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
    fn update(&mut self, _delta: &Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&self, _renderer: &mut R) -> Result<(), String> {
        Ok(())
    }
 
    fn on_event(&mut self, _event: &SdlEvent) -> Result<(), String> {
        Ok(())
    }

    // temporary routing harness
    fn event(&mut self, event: &Event<E>) -> Result<(), String> {
        match event {
            Event::Sdl(e) => self.on_event(e),
            Event::Time(dt) => self.update(dt),
            Event::Game(_) => Ok(())
        }
    }
}

pub fn run_game_loop<'a, R, G, E>(game: &mut G, renderer: &mut R, sdl_events: &mut EventPump) -> Result<(), String> 
where G: GameLoop<'a, R, E>
{
    let mut last_frame = Instant::now();
    let mut events: Events<E> = Events::new();
    loop {
        let this_frame = Instant::now();
        for event in sdl_events.poll_iter() {
            events.fire(Event::Sdl(event));
        }

        game.event(&Event::Time(this_frame.duration_since(last_frame)))?;

        for event in events.events.drain(..) {
            game.event(&event)?;
        }

        game.render(renderer)?;

        last_frame = this_frame;
    }
}

