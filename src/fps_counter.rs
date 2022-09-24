use std::time::Instant;

pub struct FpsCounter
{
    threshold: u128,
    then: Instant
}

impl FpsCounter {
    pub fn new(threshold: u128) -> Self {
        return FpsCounter {
            threshold,
            then: Instant::now()
        };
    }

    pub fn on_frame(&mut self) {
        let now = Instant::now();

        let frame_duration = now.duration_since(self.then).as_millis();
        if frame_duration > self.threshold
        {
            println!("Slow frame: took {frame_duration} between frames");
        }

        self.then = now;
    }
}