use crate::audio::instrument::*;
use crate::audio::notes::*;
use crate::audio::tempo::Phrase;
use crate::audio::tempo::Tempo;
use crate::events::Events;

pub fn countdown(events: &mut Events) {
    let tempo = Tempo::new(4, 100);

    events.fire(tempo.using(&KICK, 0)
        .play(1.0, 0.5, C2)
        .on(&RIM).play(1.5, 0.25, A0)
        .on(&RIM).play(1.75, 0.25, A0)
        .on(&KICK).play(2.0, 0.5, C2)
        .on(&HIHAT).play(2.5, 0.5, A0)
        .on(&KICK).play(3.0, 0.5, C2)
        .on(&HIHAT).play(3.5, 0.5, A0)
        .on(&CRASH).play(4.0, 1.0, A0)
        .build());
}