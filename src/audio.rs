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
pub struct PlayNote {
    pub pitch: f32,
    pub volume: f32
}

pub fn initialise_audio(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<Channel>, String> {
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        Channel {
            phase_inc: 880.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.00,
            waveform: Waveform::Pulse(0.1)
        }
    }).unwrap();

    // Start playback
    audio_device.resume();
    Ok(audio_device)
}

pub fn play_note(device: &mut AudioDevice<Channel>, &PlayNote{ pitch, volume}: &PlayNote) {
    *device.lock() = Channel {
        phase_inc: pitch / 48000.0,
        phase: 0.0,
        volume,
        waveform: Waveform::Sine
    }
}

pub struct Channel {
    phase_inc: f32,
    phase: f32,
    volume: f32,
    waveform: Waveform
}

impl AudioCallback for Channel {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = self.waveform.amplitude(self.phase) * self.volume;
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

enum Waveform {
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
