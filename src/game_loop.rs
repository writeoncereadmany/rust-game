use core::any::*;

use std::collections::VecDeque;
use std::time::{Instant, Duration};

use sdl2::EventPump;
use sdl2::event::Event as SdlEvent;

use component_derive::Event;

#[derive(Event)]
pub struct Cleanup;

pub trait EventTrait: Any {}

impl EventTrait for SdlEvent {}
impl EventTrait for Duration {}

pub struct Event(Box<dyn Any>);

impl Event {
    fn new<E: EventTrait>(event: E) -> Self {
        Event(Box::new(event))
    }

    pub fn unwrap<E: EventTrait>(&self) -> Option<&E> {
        let Event(event) = self;
        event.downcast_ref()
    }

    pub fn apply<E: EventTrait, O>(&self, f: impl FnMut(&E) -> O) -> Option<O> {
        self.unwrap().map(f)
    }
}

pub struct Events {
    events: VecDeque<Event>
}

impl Events {

    fn new() -> Self {
        Events{ events: VecDeque::new() }
    }

    pub fn fire<E: EventTrait>(&mut self, event: E) {
        self.events.push_back(Event::new(event));
    }
}

pub trait GameLoop<'a, R>
{
    fn render(&self, _renderer: &mut R) -> Result<(), String> {
        Ok(())
    }
 
    fn event(&mut self, _event: &Event, _events: &mut Events) -> Result<(), String> {
        Ok(())
    }
}

pub fn run_game_loop<'a, R, G>(mut game: G, renderer: &mut R, sdl_events: &mut EventPump, updates_per_frame: u32) -> Result<(), String> 
where G: GameLoop<'a, R>
{
    let mut last_frame = Instant::now();
    let mut events: Events = Events::new();
    let cleanup = Event::new(Cleanup);
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

