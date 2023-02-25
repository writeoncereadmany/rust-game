use crate::audio::notes::*;
use crate::audio::instrument::BASS;
use crate::audio::tempo::Tempo;
use crate::events::Events;

fn bass_bar(r: f32, a: f32, b: f32) -> Vec<(f32, f32, f32)> {
    vec![(0.0, 1.0, r), (1.5, 1.0, r), (3.0, 0.5, a), (3.5, 0.5, b)]
}

pub fn stand_by_me(events: &mut Events) {
    let bassline = Tempo::new(4, 120).using(&BASS, 0)
        .bar(1).phrase(1.0, bass_bar(A2, E2, Ab2))
        .bar(2).phrase(1.0, bass_bar(A2, A2, Ab2))
        .bar(3).phrase(1.0, bass_bar(Fs2, E2, E2))
        .bar(4).phrase(1.0, bass_bar(Fs2, Fs2, E2))
        .bar(5).phrase(1.0, bass_bar(D2, D2, Fs2))
        .bar(6).phrase(1.0, bass_bar(E2, E2, Ab2))
        .bar(7).phrase(1.0, bass_bar(A2, E2, Ab2))
        .bar(8).phrase(1.0, bass_bar(A2, E2, Ab2))
        .build();

    events.fire(bassline);
}