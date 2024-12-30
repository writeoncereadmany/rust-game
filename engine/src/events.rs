use core::any::*;

use sdl2::event::Event as SdlEvent;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::marker::PhantomData;
use std::time::{Duration, Instant};

pub trait EventTrait: Any {}

impl EventTrait for SdlEvent {}
impl EventTrait for Duration {}

pub trait EventTrait2 {
    fn apply<Event: 'static, O>(&self, f: impl FnMut(&Event) -> O) -> Option<O>;

    fn dispatch<W: 'static>(&self, dispatcher: &Dispatcher<W>, world: &mut W, events: &mut Events);
}

pub struct Dispatcher<W> {
    functions: HashMap<TypeId, Box<dyn Any>>,
    type_marker: PhantomData<W>
}

impl <W: 'static> Dispatcher<W> {
    fn register<Event: EventTrait2 + 'static>(&mut self, f: fn(&Event, &mut W, &mut Events)) {
            self.functions.entry(TypeId::of::<Event>())
                .or_insert(Box::new(Vec::<fn(&Event, &mut W, &mut Events)>::new()))
                .downcast_mut::<Vec<fn(&Event, &mut W, &mut Events)>>()
                .map(|fs| fs.push(f));
    }

    fn dispatch<Event: EventTrait2 + 'static>(&self, event: &Event, world: &mut W, events: &mut Events) {
        if let Some(functions) = self.functions.get(&TypeId::of::<Event>())
            .map(|fs| fs.downcast_ref::<Vec<fn(&Event, &mut W, &mut Events)>>())
            .flatten() {
            for function in functions {
                function(event, world, events);
            }
        }
    }
}

impl EventTrait2 for SdlEvent {
    fn apply<Event: 'static, O>(&self, f: impl FnMut(&Event) -> O) -> Option<O> {
        (self as &dyn Any).downcast_ref().map(f)
    }

    fn dispatch<W: 'static>(&self, dispatcher: &Dispatcher<W>, world: &mut W, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

#[derive(Debug)]
pub struct Event(Box<dyn Any>);

impl Event {
    pub fn new<E: EventTrait>(event: E) -> Self {
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

struct TimerEvent {
    fires_at: Instant,
    event: Event
}

impl Eq for TimerEvent {
}

impl PartialEq for TimerEvent {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Ord for TimerEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.fires_at.cmp(&self.fires_at)
    }
}

impl PartialOrd for TimerEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Events {
    events: VecDeque<Event>,
    current_time: Instant,
    scheduled_events: BinaryHeap<TimerEvent>
}

impl Events {

    pub fn new() -> Self {
        Events{ events: VecDeque::new(), scheduled_events: BinaryHeap::new(), current_time: Instant::now() }
    }

    pub fn fire<E: EventTrait>(&mut self, event: E) {
        self.events.push_back(Event::new(event));
    }

    pub fn schedule<E: EventTrait>(&mut self, dt: Duration, event: E) {
        self.scheduled_events.push(TimerEvent { fires_at: self.current_time + dt, event: Event::new(event)})
    }

    pub fn clear_schedule(&mut self) {
        self.scheduled_events.clear();
    }

    pub fn elapse(&mut self, dt: Duration) {
        self.current_time += dt;
        while self.has_pending_events() {
            if let Some(TimerEvent { event, .. }) = self.scheduled_events.pop() {
                self.events.push_back(event);
            } else {
                break;
            } 
        }
    }

    fn has_pending_events(&self) -> bool {
        if let Some(next) = self.scheduled_events.peek() {
            next.fires_at < self.current_time
        }
        else {
            false
        }
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}