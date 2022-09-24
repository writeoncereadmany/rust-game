use std::collections::VecDeque;
use std::time::{Instant};

pub struct FpsCounter
{
    timestamp_queue : VecDeque<Instant>,
    threshold: u128,
    then: Instant
}

impl FpsCounter {
    pub fn new(threshold: u128) -> Self {
        return FpsCounter {
            timestamp_queue: VecDeque::new(),
            threshold,
            then: Instant::now()
        };
    }

    pub fn on_frame(&mut self) {
        let now = Instant::now();
        while self.timestamp_queue.front().map(|&then| now.duration_since(then).as_millis() >= 1000).unwrap_or(false)
        {
            self.timestamp_queue.pop_front();
        }
        self.timestamp_queue.push_back(now);

        let frame_duration = now.duration_since(self.then).as_millis();
        if frame_duration > self.threshold
        {
            println!("Slow frame: took {frame_duration} between frames");
        }

        self.then = now;
    }

    pub fn fps(&self) -> usize {
        return self.timestamp_queue.len();
    }
}