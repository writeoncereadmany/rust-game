use std::time::Duration;

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distributions::{ Distribution, Uniform };

use sdl2::audio::{AudioDevice, AudioSpecDesired, AudioCallback};
use component_derive::Event;
use crate::events::EventTrait;

pub const A: f32 = 220.0;
pub const A_SHARP: f32 = 233.082;
pub const B_FLAT: f32 = A_SHARP;
pub const B: f32 = 246.942;
pub const C: f32 = 261.626;
pub const C_SHARP: f32 = 277.183;
pub const D_FLAT: f32 = C_SHARP;
pub const D: f32 = 293.665;
pub const D_SHARP: f32 = 311.127;
pub const E_FLAT: f32 = D_SHARP;
pub const E: f32 = 329.628;
pub const F: f32 = 349.228;
pub const F_SHARP: f32 = 369.994;
pub const G_FLAT: f32 = F_SHARP;
pub const G: f32 = 391.995;
pub const G_SHARP: f32 = 415.305;
pub const A_FLAT: f32 = G_SHARP;

#[derive(Event)]
pub struct Play(pub Note);

pub enum Note {
    Silence,
    Wave { pitch: f32, volume: f32, length: Duration },
    Noise { low: f32, high: f32, volume: f32, length: Duration }
}

pub fn initialise_audio(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<AudioPlayer>, String> {
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: None,
        channels: Some(1),  // mono
        samples: Some(128) 
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        AudioPlayer {
            rng: SmallRng::from_entropy(),
            freq: spec.freq,
            channel: Channel::Silence {}
        }
    }).unwrap();

    // Start playback
    audio_device.resume();
    Ok(audio_device)
}

pub fn play_note(device: &mut AudioDevice<AudioPlayer>, Play(note): &Play) {
    let mut device = device.lock();
    let freq = device.freq;

    let channel = match note {
        Note::Wave{ pitch, volume, .. } => {
            Channel::Wave {
                phase_inc: pitch / freq as f32,
                phase: 0.0,
                volume: *volume,
                waveform: Waveform::Triangle(0.5)
            }
        },
        Note::Noise{ low, high, volume, .. } => {
            let max_cycle = (freq as f32 / low) as u32;
            let min_cycle = (freq as f32 / high) as u32;
            let distribution : Uniform<u32> = Uniform::from(min_cycle..max_cycle);
            Channel::Noise {
                up: false,
                next_flip: 0,
                distribution,
                volume: *volume
            }
        },
        Note::Silence => Channel::Silence { }
    };
    device.set_channel(channel);
}

pub enum Channel {
    Wave {
        phase_inc: f32,
        phase: f32,
        volume: f32,
        waveform: Waveform
    }, 
    Silence { }, 
    Noise {
        up: bool,
        next_flip: u32,
        distribution: Uniform<u32>,
        volume: f32
    }
}

pub struct AudioPlayer {
    rng: SmallRng,
    freq: i32,
    channel: Channel
}

impl AudioPlayer {
    fn set_channel(&mut self, channel: Channel) {
        self.channel = channel;
    }
}

impl AudioCallback for AudioPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        match self.channel {
            Channel::Wave { phase, volume, waveform, phase_inc } => {
                let mut phase = phase;
                for x in out.iter_mut() {
                    *x = waveform.amplitude(phase) * volume;
                    phase = (phase + phase_inc) % 1.0;
                }
                self.channel = Channel::Wave { phase, volume, waveform, phase_inc };
            },
            Channel::Silence {} => {
                for x in out.iter_mut() {
                    *x = 0.0;
                }
            },
            Channel::Noise { up, mut next_flip, distribution, volume } => {
                let mut up = up;
                for x in out.iter_mut() {
                    if next_flip == 0 {
                        up = !up;
                        next_flip = distribution.sample(&mut self.rng);
                    }
                    next_flip -= 1;

                    *x = if up { volume } else { -volume }; 
                }
                self.channel = Channel::Noise { up, next_flip, distribution, volume };
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum Waveform {
    Pulse(f32),
    Sine,
    Triangle(f32)
}

impl Waveform {
    fn amplitude(&self, phase: f32) -> f32 {
        match self {
            Waveform::Pulse(duty_cycle) => if phase < *duty_cycle { 1.0 } else { -1.0 },
            Waveform::Sine => (phase * std::f32::consts::PI * 2.0).sin(),
            Waveform::Triangle(duty_cycle) => if phase < *duty_cycle {
                ((phase / duty_cycle) * 2.0) - 1.0
            } else {
                ((1.0 - phase) / (1.0 - duty_cycle)) * 2.0 - 1.0
            }
        }
    }
}
