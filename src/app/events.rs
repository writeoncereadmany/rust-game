pub enum GEvent {
    CoinCollected(u32),
    TimeLimitExpired,
    ReachedDoor,
}

pub type Events = crate::game_loop::Events<GEvent>;
pub type Event = crate::game_loop::Event<GEvent>;