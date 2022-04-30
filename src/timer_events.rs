use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::time::{Instant, Duration};

use crate::game_loop::{ Event, EventTrait, Events };

pub struct TimerEvent {
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
    fn new(now: Instant) -> Self {
        TimerEvents { current_time: now, events: BinaryHeap::new() }
    }

    fn schedule<E: EventTrait>(&mut self, dt: Duration, event: E) {
        self.events.push(TimerEvent { fires_at: self.current_time + dt, event: Event::new(event) });
    }

    fn elapse(&mut self, dt: Duration, events: &mut Events) {
        self.current_time += dt;
        loop {
            if let Some(event) = self.pop() {
                events.fire_wrapped(event);
            } 
            else {
                break;
            }
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

#[cfg(test)]
mod tests {

    use super::*;
    use crate::game_loop::EventTrait;
    use component_derive::Event;


    #[derive(Event, PartialEq, Eq, Debug)] struct TestEvent(usize);

    #[test]
    fn returns_events_once_sufficient_time_has_passed() {
        let mut timer = TimerEvents::new(Instant::now());
        timer.schedule(Duration::from_millis(300), TestEvent(1));

        assert_elapsed_events(vec![], &mut timer, Duration::from_millis(200));
        assert_elapsed_events(vec![&TestEvent(1)], &mut timer, Duration::from_millis(200));
    }

    #[test]
    fn orders_events_in_order_of_time_to_fire() {
        let mut timer = TimerEvents::new(Instant::now());
        timer.schedule(Duration::from_millis(300), TestEvent(1));
        timer.schedule(Duration::from_millis(200), TestEvent(2));
        timer.schedule(Duration::from_millis(400), TestEvent(3));

        assert_elapsed_events(
            vec![&TestEvent(2), &TestEvent(1), &TestEvent(3)], 
            &mut timer, Duration::from_millis(500)
        );
    }

    #[test]
    fn orders_events_in_order_of_time_to_fire_if_scheduled_after_time_passes() {
        let mut timer = TimerEvents::new(Instant::now());

        timer.schedule(Duration::from_millis(300), TestEvent(1));
        timer.schedule(Duration::from_millis(200), TestEvent(2));
        timer.schedule(Duration::from_millis(400), TestEvent(3));

        assert_elapsed_events(
            vec![&TestEvent(2), &TestEvent(1)], 
            &mut timer, Duration::from_millis(350)
        );

        timer.schedule(Duration::from_millis(25), TestEvent(4)); // should fire at 375
        timer.schedule(Duration::from_millis(75), TestEvent(5)); // should fire at 425


        assert_elapsed_events(
            vec![&TestEvent(4), &TestEvent(3), &TestEvent(5)], 
            &mut timer, Duration::from_millis(100)
        );
    }

    fn assert_elapsed_events<'a>(expected: Vec<&TestEvent>, timer: &'a mut TimerEvents, dt: Duration) {
        let mut events = Events::new();
        let mut popped = Vec::new();
        timer.elapse(dt, &mut events);
        loop {
            if let Some(event) = events.pop() {
                popped.push(event);
            }
            else {
                break;
            }
        }
        let actual : Vec<&TestEvent> = popped.iter().flat_map(|e| e.unwrap()).collect();
        assert_eq!(expected, actual);
    }
}