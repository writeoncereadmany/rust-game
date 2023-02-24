use std::time::Duration;
use super::instrument::*;
use super::audio::*;

pub struct Tempo {
    beats: usize,
    beat_length: f32
}

impl Tempo {
    pub fn new(beats: usize, bpm: usize) -> Tempo {
        Tempo { beats, beat_length : 60.0 / bpm as f32 }
    }

    pub fn using<'a>(&'a self, instrument: &'a Instrument, channel: usize) -> TuneBuilder<'a> {
        TuneBuilder {
            tempo: self,
            instrument,
            channel,
            bar: 1,
            notes: Vec::new()
        }
    }

    fn beat(&self, beat: f32, bar: usize, length: f32) -> (f32, f32) {
        // beats and bars count from 1, not 0
        let total_beat = ((bar - 1) * self.beats) as f32 + (beat - 1.0);
        (self.beat_length * total_beat, self.beat_length * length)
    }
}

pub struct TuneBuilder<'a> {
    tempo: &'a Tempo,
    instrument: &'a Instrument,
    channel: usize,
    bar: usize,
    notes: Vec<(Duration, Note)>
}

impl <'a> TuneBuilder<'a> {

    pub fn play(mut self, beat: f32, length: f32, pitch: f32, octave: i32) -> Self {
        let (start, len) = self.tempo.beat(beat, self.bar, length);
        self.notes.push((Duration::from_secs_f32(start), self.instrument.note(pitch, octave, len)));
        self
    }

    pub fn phrase(mut self, phrase: Vec<(f32, f32, f32, i32)>) -> Self {
        for (beat, length, pitch, octave) in phrase {
            let (start, len) = self.tempo.beat(beat, self.bar, length);
            self.notes.push((Duration::from_secs_f32(start), self.instrument.note(pitch, octave, len)));
        }
        self
    }

    pub fn bar(mut self, bar: usize) -> Self {
        self.bar = bar;
        self
    }

    pub fn build(self) -> PlayTune {
        let TuneBuilder { channel, notes, .. } = self;
        PlayTune(channel, notes)
    }
}

#[cfg(test)]
mod tests {

    use std::time::Duration;
    use super::*;
    use crate::audio::instrument::BELL;

    #[test]
    fn converts_beats_into_times() {
        let tune = Tempo::new(4, 60)
            .using(&BELL, 2)
            .play(1.0, 1.0, C, 3)
            .play(2.0, 1.0, D, 3)
            .build();

        assert_eq!(tune, PlayTune(2, vec![
            (Duration::from_secs(0), BELL.note(C, 3, 1.0)),
            (Duration::from_secs(1), BELL.note(D, 3, 1.0))
        ]));
    }

    #[test]
    fn plays_tunes_faster_with_higher_bpm() {
        let tune = Tempo::new(4, 120)
            .using(&BELL, 2)
            .play(1.0, 1.0, C, 3)
            .play(2.0, 1.0, D, 3)
            .build();

        assert_eq!(tune, PlayTune(2, vec![
            (Duration::from_millis(0), BELL.note(C, 3, 1.0)),
            (Duration::from_millis(500), BELL.note(D, 3, 1.0))
        ]));
    }

    #[test]
    fn handles_bars() {
        let tune = Tempo::new(4, 120)
            .using(&BELL, 2)
            .play(1.0, 1.0, C, 3)
            .play(2.0, 1.0, D, 3)
            .bar(2)
            .play(1.0, 1.0, E, 3)
            .play(2.0, 1.0, D, 3)
            .build();

        assert_eq!(tune, PlayTune(2, vec![
            (Duration::from_millis(0), BELL.note(C, 3, 1.0)),
            (Duration::from_millis(500), BELL.note(D, 3, 1.0)),
            (Duration::from_millis(2000), BELL.note(E, 3, 1.0)),
            (Duration::from_millis(2500), BELL.note(D, 3, 1.0)),
        ]));
    }


    #[test]
    fn handles_bars_with_different_time_signatures() {
        let tune = Tempo::new(3, 120)
            .using(&BELL, 2)
            .play(1.0, 1.0, C, 3)
            .play(2.0, 1.0, D, 3)
            .bar(2)
            .play(1.0, 1.0, E, 3)
            .play(2.0, 1.0, D, 3)
            .build();

        assert_eq!(tune, PlayTune(2, vec![
            (Duration::from_millis(0), BELL.note(C, 3, 1.0)),
            (Duration::from_millis(500), BELL.note(D, 3, 1.0)),
            (Duration::from_millis(1500), BELL.note(E, 3, 1.0)),
            (Duration::from_millis(2000), BELL.note(D, 3, 1.0)),
        ]));
    }
}