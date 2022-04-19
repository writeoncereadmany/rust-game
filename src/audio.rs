use sdl2::audio::{AudioDevice, AudioSpecDesired, AudioCallback};

pub fn initialise_audio(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<SquareWave>, String> {
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        SquareWave {
            phase_inc: 880.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.0,
            waveform: Waveform::Sine
        }
        
    }).unwrap();

    // Start playback
    audio_device.resume();
    Ok(audio_device)
}

pub struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
    waveform: Waveform
}

impl AudioCallback for SquareWave {
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
