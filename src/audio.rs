use std::collections::BinaryHeap;
use std::time::Duration;
use std::cmp::{Ord, Ordering};

use rand::SeedableRng;
use rand::rngs::SmallRng;
use rand::distributions::{ Distribution, Uniform };

use derivative::Derivative;

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

#[derive(Event)]
pub struct PlayTune(pub Vec<(Duration, Note)>);

#[derive(Clone, Copy, Debug)]
pub enum Note {
    Silence,
    Wave { pitch: f32, volume: f32, length: Duration },
    Noise { low: f32, high: f32, volume: f32, length: Duration }
}

pub fn initialise_audio(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<AudioPlayer>, String> {
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),  // mono
        samples: Some(128) 
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        AudioPlayer {
            rng: SmallRng::from_entropy(),
            freq: spec.freq,
            cycles: 0,
            queue: BinaryHeap::new(),
            channel: Channel::Silence {}
        }
    }).unwrap();

    // Start playback
    audio_device.resume();
    Ok(audio_device)
}

pub fn play_note(device: &mut AudioDevice<AudioPlayer>, Play(note): &Play) {
    device.lock().play(note);
}

pub fn play_tune(device: &mut AudioDevice<AudioPlayer>, PlayTune(tune): &PlayTune) {
    device.lock().cue(tune);
}

pub enum Channel {
    Wave {
        phase_inc: f32,
        phase: f32,
        volume: f32,
        cycles_remaining: u32,
        waveform: Waveform
    }, 
    Silence, 
    Noise {
        up: bool,
        next_flip: u32,
        distribution: Uniform<u32>,
        volume: f32,
        cycles_remaining: u32,
    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Cue {
    #[derivative(PartialOrd(compare_with="partial_reversed"), Ord(compare_with="reversed"))]
    start_at: u64,
    #[derivative(PartialEq="ignore", PartialOrd="ignore", Ord="ignore")]
    note: Note
}

pub fn reversed<T: Ord>(first: &T, second: &T) -> Ordering {
    second.cmp(first)
}


pub fn partial_reversed<T: Ord>(first: &T, second: &T) -> Option<Ordering> {
    Some(reversed(first, second))
}


pub struct AudioPlayer {
    rng: SmallRng,
    freq: i32,
    cycles: u64,
    queue: BinaryHeap<Cue>,
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
        for x in out.iter_mut() {

            if let Some(note) = self.due() {
                self.play(&note);
            }

            match self.channel {
                Channel::Wave { phase, volume, waveform, phase_inc, cycles_remaining } => {
                    *x = waveform.amplitude(phase) * volume;
                    if cycles_remaining > 0 {
                        self.channel = Channel::Wave { 
                            phase: (phase + phase_inc) % 1.0, 
                            volume, 
                            waveform, 
                            phase_inc, 
                            cycles_remaining: cycles_remaining - 1 
                        };
                    }
                    else {
                        self.channel = Channel::Silence;
                    }
                },
                Channel::Silence {} => {
                    *x = 0.0;
                },
                Channel::Noise { up, next_flip, distribution, volume, cycles_remaining } => {
                    *x = if up { volume } else { -volume }; 

                    if cycles_remaining > 0 {
                        self.channel = Channel::Noise { 
                            up: if next_flip == 0 { !up } else { up } , 
                            next_flip: if next_flip == 0 { distribution.sample(&mut self.rng) } else { next_flip - 1 }, 
                            distribution, 
                            volume,
                            cycles_remaining: cycles_remaining - 1
                        };
                    } else {
                        self.channel = Channel::Silence
                    }
                }
            }
            self.cycles += 1;
        }
    } 
}

impl AudioPlayer {

    fn cue(&mut self, tune: &Vec<(Duration, Note)>) {
        self.queue.clear();
        for (delay, note) in tune {
            let cycles_before_start = (delay.as_secs_f64() * self.freq as f64) as u64;
            let start_at = cycles_before_start + self.cycles;
            self.queue.push(Cue { start_at, note: *note });
        }
    }

    fn due(&mut self) -> Option<Note> {
        if match self.queue.peek() {
            Some(cue) if cue.start_at <= self.cycles => true,
            _otherwise => false
        } {
            self.queue.pop().map(|cue| cue.note)
        } else {
            Option::None
        }
    }

    fn play(&mut self, note: &Note) {
        let freq = self.freq;

        let current_phase = match self.channel {
            Channel::Wave { phase, ..} => phase,
            _ => 0.0
        };
    
        let channel = match note {
            Note::Wave{ pitch, volume, length } => {
                let cycles_remaining = (length.as_secs_f64() * freq as f64) as u32;
                Channel::Wave {
                    phase_inc: pitch / freq as f32,
                    phase: current_phase,
                    volume: *volume,
                    waveform: Waveform::Triangle(0.5),
                    cycles_remaining
                }
            },
            Note::Noise{ low, high, volume, length } => {
                let max_cycle = (freq as f32 / low) as u32;
                let min_cycle = (freq as f32 / high) as u32;
                let distribution : Uniform<u32> = Uniform::from(min_cycle..max_cycle);
                let cycles_remaining = (length.as_secs_f64() * freq as f64) as u32;
                Channel::Noise {
                    up: false,
                    next_flip: 0,
                    distribution,
                    volume: *volume,
                    cycles_remaining
                }
            },
            Note::Silence => Channel::Silence
        };
        self.set_channel(channel);
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_iterate_only_over_subset(){
        let mut heap = BinaryHeap::new();

        heap.push(Cue { start_at: 1, note: Note::Silence });
        heap.push(Cue { start_at: 2, note: Note::Silence });
        heap.push(Cue { start_at: 3, note: Note::Silence });
        heap.push(Cue { start_at: 4, note: Note::Silence });

        assert_eq!(heap.pop().unwrap().start_at, 1);
        assert_eq!(heap.pop().unwrap().start_at, 2);
        assert_eq!(heap.pop().unwrap().start_at, 3);
        assert_eq!(heap.pop().unwrap().start_at, 4);
        assert_eq!(heap.pop(), None);
    }
}