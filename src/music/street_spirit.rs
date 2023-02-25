use crate::events::Events;
use crate::audio::tempo::Tempo;
use crate::audio::instrument::SW;

use super::phrases::tuplet;
use crate::audio::notes::*;


fn riff(b: f32, m: f32, r: f32, x: f32, y: f32, z: f32, t: f32) -> Vec<(f32, f32, f32)> {
    tuplet(0.25, vec![b, r, t, r, z, r, t, r, y, r, t, r, x, r, m, r])
}

fn riff_a() -> Vec<(f32, f32, f32)> {
    riff(A2, E3, A3, B3, C4, D4, E4)
}

fn riff_e() -> Vec<(f32, f32, f32)> {
    riff(E2, E3, G3, B3, C4, D4, E4)
}

fn riff_c() -> Vec<(f32, f32, f32)> {
    riff(C3, E3, G3, B3, C4, D4, E4)
}

pub fn street_spirit(events: &mut Events) {
    let riffs = Tempo::new(4, 120).using(&SW, 0)
        .bar(1).phrase(1.0, riff_a())
        .bar(2).phrase(1.0, riff_a())
        .bar(3).phrase(1.0, riff_a())
        .bar(4).phrase(1.0, riff_a())
        .bar(5).phrase(1.0, riff_e())
        .bar(6).phrase(1.0, riff_e())
        .bar(7).phrase(1.0, riff_a())
        .bar(8).phrase(1.0, riff_a())
        .bar(9).phrase(1.0, riff_c())
        .bar(10).phrase(1.0, riff_e())
        .bar(11).phrase(1.0, riff_a())
        .bar(12).phrase(1.0, riff_a())
        .bar(13).phrase(1.0, riff_c())
        .bar(14).phrase(1.0, riff_e())
        .bar(15).phrase(1.0, riff_a())
        .bar(16).phrase(1.0, riff_a())
        .build();

    events.fire(riffs);
}