use std::collections::VecDeque;
use std::time::{Instant};

pub struct FpsCounter
{
    timestamp_queue : VecDeque<Instant>
}

impl FpsCounter {
    pub fn new() -> Self {
        return FpsCounter {
            timestamp_queue: VecDeque::new()
        };
    }

    pub fn on_frame(&mut self) {
        let now = Instant::now();
        self.timestamp_queue.retain(|&then| now.duration_since(then).as_millis() < 1_000);
        self.timestamp_queue.push_back(now);
    }

    pub fn fps(&self) -> usize {
        return self.timestamp_queue.len();
    }
}