use crate::game_loop::Evento;

pub enum GEvent {
    CoinCollected(u32),
    TimeLimitExpired,
    ReachedDoor,
}

impl Evento for GEvent {}