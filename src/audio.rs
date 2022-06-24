use sdl2::audio::{AudioDevice, AudioSpecDesired, AudioCallback};

pub const A: f64 = 440.0;
pub const A_SHARP: f64 = (2.0: f64).powf(1.0/12.0) * A;
pub const B_FLAT: f64 = A_SHARP;
pub const B: f64 = (2.0: f64).powf(2.0/12.0) * A;
pub const C: f64 = (2.0: f64).powf(3.0/12.0) * A;
pub const C_SHARP: f64 = (2.0: f64).powf(4.0/12.0) * A;
pub const D_FLAT: f64 = C_SHARP;
pub const D: f64 = (2.0: f64).powf(5.0/12.0) * A;
pub const D_SHARP: f64 = (2.0: f64).powf(6.0/12.0) * A;
pub const E_FLAT: f64 = D_SHARP;
pub const E: f64 = (2.0: f64).powf(7.0/12.0) * A;
pub const F: f64 = (2.0: f64).powf(8.0/12.0) * A;
pub const F_SHARP: f64 = (2.0: f64).powf(9.0/12.0) * A;
pub const G_FLAT: f64 = F_SHARP;
pub const G: f64 = (2.0: f64).powf(10.0/12.0) * A;
pub const G_SHARP: f64 = (2.0: f64).powf(11.0/12.0) * A;
pub const A_FLAT: f64 = G_SHARP;

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
