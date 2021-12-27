pub enum GEvent {
    CoinCollected(u32),
}

pub type Events = crate::game_loop::Events<GEvent>;
pub type Event = crate::game_loop::Event<GEvent>;