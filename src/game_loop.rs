use core::any::*;

use std::collections::VecDeque;
use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event as SdlEvent;

pub struct Cleanup;

pub trait Evento: Any {}

impl Evento for SdlEvent {}
impl Evento for Duration {}
impl Evento for Cleanup {}

pub struct Eventy(Box<dyn Any>);

impl Eventy {
    fn new<E: Evento>(event: E) -> Self {
        Eventy(Box::new(event))
    }

    pub fn unwrap<'a, E: Evento>(&'a self) -> Option<&'a E> {
        let Eventy(event) = self;
        event.downcast_ref()
    }
}

pub enum Event<E> {
    Sdl(SdlEvent),
    Time(Duration),
    Game(E),
    Cleanup,
}

pub struct Events {
    events: VecDeque<Eventy>
}

impl Events {

    fn new() -> Self {
        Events{ events: VecDeque::new() }
    }

    pub fn fire<E: Evento>(&mut self, event: E) {
        self.events.push_back(Eventy::new(event));
    }
}

pub trait GameLoop<'a, R>
{
    fn render(&self, _renderer: &mut R) -> Result<(), String> {
        Ok(())
    }
 
    fn event(&mut self, _event: &Eventy, _events: &mut Events) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<'a, R, G>(mut game: G, renderer: &mut R, sdl_events: &mut EventPump, updates_per_frame: u32) -> Result<(), String> 
where G: GameLoop<'a, R>
{
    let mut last_frame = Instant::now();
    let mut events: Events = Events::new();
    let cleanup = Eventy::new(Cleanup);
    loop {
        let this_frame = Instant::now();
        for event in sdl_events.poll_iter() {
            events.fire(event);
        }

        for _ in 0..updates_per_frame {
            events.fire(this_frame.duration_since(last_frame).div_f64(updates_per_frame as f64));

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
        }

        game.event(&cleanup, &mut events)?;

        game.render(renderer)?;

        last_frame = this_frame;
    }
}

