use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Instant, Duration};

use component_derive::Event;

use crate::events::{ Event, EventTrait, Events };

#[derive(Event)]
pub struct FutureEvent {
    delay: Duration,
    event: Event
}

impl FutureEvent {
    fn in_ms<E: EventTrait>(ms: u64, event: E) -> Self {
        FutureEvent { delay: Duration::from_millis(ms), event: Event::new(event) }
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

pub struct TimerEvents {
    current_time: Instant,
    events: BinaryHeap<TimerEvent>
}

impl TimerEvents {
    pub fn new(now: Instant) -> Self {
        TimerEvents { current_time: now, events: BinaryHeap::new() }
    }

    pub fn schedule(&mut self, event: &FutureEvent) {
        self.events.push(TimerEvent { fires_at: self.current_time + event.delay, event: event.event});
    }

    pub fn elapse(&mut self, dt: Duration, events: &mut Events) {
        self.current_time += dt;
        while let Some(event) = self.pop() {
            events.fire_wrapped(event);
        }
    }

    fn has_pending_events(&self) -> bool {
        if let Some(next) = self.events.peek() {
            return next.fires_at < self.current_time;
        }
        else {
            false
        }
    }

    fn pop(&mut self) -> Option<Event> {
        if self.has_pending_events() {
            if let Some(TimerEvent { event, .. }) = self.events.pop() {
                return Some(event);
            }
        }
        None
    }
}

