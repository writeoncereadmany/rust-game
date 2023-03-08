use super::audio::*;
use super::notes::*;

pub const BELL : Instrument = Instrument::Pitched { waveform: Waveform::Triangle(0.5), enveloper: decay(0.5, 0.25)};
pub const SW : Instrument = Instrument::Pitched { waveform: Waveform::Pulse(0.5), enveloper: constant(0.05)};
pub const BASS : Instrument = Instrument::Pitched { waveform: Waveform::Triangle(0.85), enveloper: constant(0.15)};
pub const FLUTE: Instrument = Instrument::Pitched { waveform: Waveform::Sine, enveloper: ADSR { attack: 0.1, decay: 0.2, release: 0.3, peak: 0.25, sustained: 0.15 }};

pub const CYMBAL: Instrument = Instrument::Percussion { low: A4, high: A6, enveloper: decay(0.5, 0.25)};
pub const CLAP: Instrument = Instrument::Percussion { low: A6, high: A8, enveloper: decay(0.1, 0.15)};
pub const RIM: Instrument = Instrument::Percussion { low: A8, high: A9, enveloper: decay(0.1, 0.15)};
pub const HIHAT: Instrument = Instrument::Percussion { low: C7, high: C9, enveloper: decay(0.25, 0.15)};
pub const CRASH: Instrument = Instrument::Percussion { low: C7, high: C9, enveloper: decay(1.0, 0.15)};
pub const KICK: Instrument = Instrument::Pitched { waveform: Waveform::Triangle(0.5), enveloper: decay(0.15, 0.55)};

#[derive(Debug, PartialEq)]
pub struct ADSR { attack: f32, decay: f32, release: f32, peak: f32, sustained: f32 }

const fn decay(decay: f32, volume: f32) -> ADSR {
    ADSR { attack: 0.0, decay, release: 0.0, peak: volume, sustained: 0.0 }
}

const fn constant(volume: f32) -> ADSR {
    ADSR { attack: 0.0, decay: 0.0, release: 0.0, peak: 0.0, sustained: volume }
}

impl ADSR {
    fn envelope(&self, duration: f32) -> EnvSpec {
        match self {
            &ADSR { attack, decay, release, peak, sustained } => {
                let sustain = (duration - (attack + decay)).max(0.0);
                EnvSpec::vols(vec![
                (0.0, 0.0),
                (attack, peak),
                (decay, sustained),
                (sustain, sustained),
                (release, 0.0)
            ])}
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instrument {
    Pitched {
    waveform: Waveform, 
    enveloper: ADSR,
    }, 
    Percussion {
        low: f32,
        high: f32,
        enveloper: ADSR
    }
}

impl Instrument {
    pub fn note(&self, pitch: f32, duration: f32) ->  Note {
        match &self {
            Instrument::Pitched { waveform, enveloper } => Note::Wave { pitch, waveform: *waveform, envelope: enveloper.envelope(duration) },
            Instrument::Percussion { low, high, enveloper } => Note::Noise { low: *low, high: *high, envelope: enveloper.envelope(duration) }
        }
    }
}