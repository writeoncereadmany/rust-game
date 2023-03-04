use crate::audio::instrument::*;
use crate::audio::notes::*;
use crate::audio::tempo::Tempo;
use crate::events::Events;

use super::phrases::tuplet;

pub fn countdown(events: &mut Events) {
    let tempo = Tempo::new(4, 100);

    events.fire(tempo.using(&KICK, 0)
        .play(1.0, 0.5, C2)
        .on(&HIHAT).play(1.5, 0.5, A0)
        .on(&KICK).play(2.0, 0.5, C2)
        .on(&HIHAT).play(2.5, 0.5, A0)
        .on(&KICK).play(3.0, 0.5, C2)
        .on(&HIHAT).play(3.5, 0.5, A0)
        .on(&CRASH).play(4.0, 1.0, A0)
        .build());

    events.fire(tempo.using(&BASS, 1)
        .play(1.0, 1.0, F2)
        .play(2.0, 1.0, G2)
        .play(3.0, 1.0, C3)
        .play(4.0, 1.0, C2)
        .build());

    events.fire(Tempo::new(4, 100).using(&SW, 2)
        .phrase(1.0, tuplet(1.0 / 6.0, vec![F4, G4, C5, G4, B4, C5]))
        .phrase(2.0, tuplet(1.0 / 6.0, vec![G4, B4, C5, B4, C5, G5]))
        .phrase(3.0, tuplet(1.0 / 6.0, vec![B4, C5, G5, C5, G5, C6]))
        .play(4.0, 1.0, C5)
        .build());
}