use std::time::Duration;
use std::cmp::max;

struct TimeBuffer<'a> {
    current_value : Option<&'a str>,
    until_expiry : Duration
}

impl <'a> TimeBuffer<'a> {
    fn new() -> Self {
        TimeBuffer {
            current_value: None,
            until_expiry: Duration::ZERO
        }
    }

    fn put(&mut self, value: &'a str, expires_in: Duration) {
        self.current_value = Some(value);
        self.until_expiry = max(self.until_expiry, expires_in);
    }

    fn elapse(&mut self, duration: Duration) {
        self.until_expiry = self.until_expiry.saturating_sub(duration);

        if self.until_expiry <= Duration::ZERO {
            self.current_value = None;
        }
    }

    fn peek(&self) -> Option<&'a str> {
        self.current_value
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn buffer_starts_off_empty() {
        let buffer = TimeBuffer::new();
        assert_eq!(buffer.peek(), None);
    }

    #[test]
    fn after_adding_to_buffer_it_holds_that_value() {
        let mut buffer = TimeBuffer::new();
        buffer.put("Hello", Duration::from_millis(20));
        assert_eq!(buffer.peek(), Some("Hello"));
    }

    #[test]
    fn value_in_buffer_expires_after_timeout() {
        let mut buffer = TimeBuffer::new();
        buffer.put("Hello", Duration::from_millis(20));
        buffer.elapse(Duration::from_millis(30));
        assert_eq!(buffer.peek(), None);
    }

    #[test]
    fn expiry_time_can_be_extended() {
        let mut buffer = TimeBuffer::new();
        buffer.put("Hello", Duration::from_millis(10));
        buffer.put("Hello", Duration::from_millis(20));

        // valid for 20ms, so still valid after 15ms... 
        buffer.elapse(Duration::from_millis(15));
        assert_eq!(buffer.peek(), Some("Hello"));

        // but not after a further 10ms
        buffer.elapse(Duration::from_millis(10));
        assert_eq!(buffer.peek(), None);
    }

    #[test]
    fn expiry_time_cannot_be_overridden_with_smaller_value() {
        let mut buffer = TimeBuffer::new();
        buffer.put("Hello", Duration::from_millis(20));
        buffer.put("Hello", Duration::from_millis(10));

        // valid for 20ms, so still valid after 15ms... 
        buffer.elapse(Duration::from_millis(15));
        assert_eq!(buffer.peek(), Some("Hello"));

        // but not after a further 10ms
        buffer.elapse(Duration::from_millis(10));
        assert_eq!(buffer.peek(), None);
    }

}