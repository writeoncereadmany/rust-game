use super::audio::*;
use super::notes::*;

pub const BELL : Instrument = Instrument { waveform: Waveform::Triangle(0.5), enveloper: Enveloper::Decay { decay: 0.5, volume: 0.25 }};
pub const FLUTE: Instrument = Instrument { waveform: Waveform::Sine, enveloper: Enveloper::ADSR { attack: 0.1, decay: 0.2, release: 0.3, peak: 0.25, sustained: 0.15 }};
pub const CYMBAL: Percussion = Percussion { low: A4, high: A6, enveloper: Enveloper::Decay { decay: 0.5, volume: 0.25 }};
pub const BASS: Instrument = Instrument { waveform: Waveform::Pulse(0.7), enveloper: Enveloper::ADSR { attack: 0.1, decay: 0.1, release: 0.2, peak: 0.2, sustained: 0.1 }};

#[derive(Debug, PartialEq)]
pub enum Enveloper {
    Decay { decay: f32, volume: f32 },
    ADSR { attack: f32, decay: f32, release: f32, peak: f32, sustained: f32 }
}

impl Enveloper {
    fn envelope(&self, duration: f32) -> EnvSpec {
        match self {
            &Enveloper::ADSR { attack, decay, release, peak, sustained } => {
                let sustain = (duration - (attack + decay)).max(0.0);
                EnvSpec::vols(vec![
                (0.0, 0.0),
                (attack, peak),
                (decay, sustained),
                (sustain, sustained),
                (release, 0.0)
            ])},
            &Enveloper::Decay { decay, volume } => {
                EnvSpec::vols(vec![
                    (0.0, volume),
                    (decay, 0.0)
                ])
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instrument {
    waveform: Waveform, 
    enveloper: Enveloper,
}

impl Instrument {
    pub fn note(&self, pitch: f32, duration: f32) ->  Note {
        // A as defined in audio.rs is A3, and so on from there, so normalise to that:
        Note::Wave { pitch, waveform: self.waveform, envelope: self.enveloper.envelope(duration) }
    }
}

pub struct Percussion {
    low: f32,
    high: f32,
    enveloper: Enveloper
}

impl Percussion {
    pub fn note(&self, duration: f32) ->  Note {
        Note::Noise { low: self.low, high: self.high, envelope: self.enveloper.envelope(duration) }
    }
}