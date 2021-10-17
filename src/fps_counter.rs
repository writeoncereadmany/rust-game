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
        while self.timestamp_queue.front().map(|&then| now.duration_since(then).as_millis() >= 1000).unwrap_or(false)
        {
            self.timestamp_queue.pop_front();
        }
        self.timestamp_queue.push_back(now);
    }

    pub fn fps(&self) -> usize {
        return self.timestamp_queue.len();
    }
}