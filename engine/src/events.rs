use entity::Entities;
use sdl2::event::Event as SdlEvent;
use std::any::{Any, TypeId};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::time::{Duration, Instant};

pub trait EventTrait {
    fn as_any(&self) -> &dyn Any;

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events);
}

pub struct Event(Box<dyn EventTrait>);

impl Event {
    pub fn new<E: EventTrait + 'static>(event: E) -> Self {
        Event(Box::new(event))
    }

    pub fn unwrap<E: EventTrait + 'static>(&self) -> Option<&E> {
        let Event(event) = self;
        event.as_any().downcast_ref()
    }

    pub fn apply<E:EventTrait + 'static, O>(&self, f: impl FnMut(&E) -> O) -> Option<O> {
        self.unwrap().map(f)
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        let Event(event) = self;
        event.dispatch(dispatcher, world, events);
    }
}

pub struct Dispatcher {
    functions: HashMap<TypeId, Box<dyn Any>>
}

impl Dispatcher {
    fn new() -> Self {
        Dispatcher { functions: HashMap::new() }
    }

    fn register<Event: EventTrait + 'static>(&mut self, f: fn(&Event, &mut Entities, &mut Events)) {
        self.functions.entry(TypeId::of::<Event>())
            .or_insert(Box::new(Vec::<fn(&Event, &mut Entities, &mut Events)>::new()))
            .downcast_mut::<Vec<fn(&Event, &mut Entities, &mut Events)>>()
            .map(|fs| fs.push(f));
    }

    pub fn dispatch<Event: EventTrait + 'static>(&self, event: &Event, world: &mut Entities, events: &mut Events) {
        if let Some(functions) = self.functions.get(&TypeId::of::<Event>())
            .map(|fs| fs.downcast_ref::<Vec<fn(&Event, &mut Entities, &mut Events)>>())
            .flatten() {
            for function in functions {
                function(event, world, events);
            }
        }
    }
}

impl EventTrait for SdlEvent {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
    }
}

impl EventTrait for Duration {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dispatch(&self, dispatcher: &Dispatcher, world: &mut Entities, events: &mut Events) {
        dispatcher.dispatch(self, world, events);
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
        Events { events: VecDeque::new(), scheduled_events: BinaryHeap::new(), current_time: Instant::now() }
    }

    pub fn fire<E: EventTrait + 'static>(&mut self, event: E) {
        self.events.push_back(Event::new(event));
    }

    pub fn schedule<E: EventTrait + 'static>(&mut self, dt: Duration, event: E) {
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

    pub fn dispatch(&mut self, dispatcher: &Dispatcher, entities: &mut Entities) {
        while !self.events.is_empty() {
            self.events.pop_front().map(|event| event.dispatch(dispatcher, entities, self));
        }
    }

    pub fn pop(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use crate as engine;
    use super::*;

    pub use component_derive::{Event, Variable};
    use entity::entity;

    #[derive(Variable, Clone, Debug, PartialEq, Eq)]
    struct Score(u32);

    #[derive(Variable, Clone, Debug, PartialEq, Eq)]
    struct EventCount(u32);

    #[derive(Event)]
    struct Points(u32);

    #[derive(Event)]
    struct DoublePoints(u32);

    #[derive(Event)]
    struct NoHandler;

    #[test]
    fn handle_events_via_dispatcher() {
        let mut entities = Entities::new();
        let mut dispatcher = Dispatcher::new();
        let mut events = Events::new();

        dispatcher.register(|&Points(p), entities, events| entities.apply(|Score(s)| Score(p + s)));
        dispatcher.register(|&Points(_), entities, events| entities.apply(|EventCount(c)| EventCount(c + 1)));

        dispatcher.register(|&DoublePoints(d), entities, events| entities.apply(|Score(s)| {
            events.fire(Points(d));
            Score(s + d)
        }));
        dispatcher.register(|&DoublePoints(_), entities, events| entities.apply(|EventCount(c)| EventCount(c + 1)));

        entities.spawn(entity().with(Score(0)).with(EventCount(0)));

        events.fire(Points(20));
        events.fire(DoublePoints(13));
        events.fire(NoHandler);

        events.dispatch(&dispatcher, &mut entities);

        assert_eq!(entities.collect::<Score>(), vec!(Score(46)));
        assert_eq!(entities.collect::<EventCount>(), vec!(EventCount(3)));
    }

    #[test]
    fn handle_events_via_application() {
        let mut total_score = 0;

        let event_1 = Event::new(Points(20));
        let event_2 = Event::new(DoublePoints(30));

        event_1.apply(|Points(s)| total_score += s);
        assert_eq!(total_score, 20);

        event_2.apply(|Points(s)| total_score += s);
        assert_eq!(total_score, 20);

        event_1.apply(|DoublePoints(s)| total_score += 2 * s);
        assert_eq!(total_score, 20);

        event_2.apply(|DoublePoints(s)| total_score += 2 * s);
        assert_eq!(total_score, 80);
    }
}