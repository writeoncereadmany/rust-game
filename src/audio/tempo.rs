use std::time::Duration;
use super::instrument::*;
use super::audio::*;

struct Tempo {
    beats: usize,
    beat_length: f32
}

impl Tempo {
    fn new(beats: usize, bpm: usize) -> Tempo {
        Tempo { beats, beat_length : 60.0 / bpm as f32 }
    }

    fn using<'a>(&'a self, instrument: &'a Instrument, channel: usize) -> TuneBuilder<'a> {
        TuneBuilder {
            tempo: self,
            instrument,
            channel,
            notes: Vec::new()
        }
    }

    fn beat(&self, beat: f32, length: f32) -> (f32, f32) {
        // beats count from 1, not 0
        (self.beat_length * (beat - 1.0), self.beat_length * length)
    }
}

struct TuneBuilder<'a> {
    tempo: &'a Tempo,
    instrument: &'a Instrument,
    channel: usize, 
    notes: Vec<(Duration, Note)>
}

impl <'a> TuneBuilder<'a> {

    fn play(mut self, pitch: f32, octave: i32, beat: f32, length: f32) -> Self {
        let (start, len) = self.tempo.beat(beat, length);
        self.notes.push((Duration::from_secs_f32(start), self.instrument.note(pitch, octave, len)));
        self
    }

    fn build(self) -> PlayTune {
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
            .play(C, 3, 1.0, 1.0)
            .play(D, 3, 2.0, 1.0)
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
            .play(C, 3, 1.0, 1.0)
            .play(D, 3, 2.0, 1.0)
            .build();

        assert_eq!(tune, PlayTune(2, vec![
            (Duration::from_millis(0), BELL.note(C, 3, 1.0)),
            (Duration::from_millis(500), BELL.note(D, 3, 1.0))
        ]));
    }
}