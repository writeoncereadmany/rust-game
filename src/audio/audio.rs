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
pub const As: f32 = 233.082;
pub const Bb: f32 = As;
pub const B: f32 = 246.942;
pub const C: f32 = 261.626;
pub const Cs: f32 = 277.183;
pub const Db: f32 = Cs;
pub const D: f32 = 293.665;
pub const Ds: f32 = 311.127;
pub const Eb: f32 = Ds;
pub const E: f32 = 329.628;
pub const F: f32 = 349.228;
pub const Fs: f32 = 369.994;
pub const Gb: f32 = Fs;
pub const G: f32 = 391.995;
pub const Gs: f32 = 415.305;
pub const Ab: f32 = Gs / 2.0;

#[derive(Event)]
pub struct PlayTune(pub usize, pub Vec<(Duration, Note)>);

#[derive(Clone, Debug)]
pub enum Note {
    Silence,
    Wave { pitch: f32, envelope: EnvSpec, waveform: Waveform },
    Noise { low: f32, high: f32, envelope: EnvSpec }
}

trait Envelope {
    fn volume(&self, cycle: u32) -> f32;
    fn complete(&self, cycle: u32) -> bool;
}

struct EnvelopeSegment {
    start: u32,
    start_volume: f32,
    end: u32,
    end_volume: f32
}

impl Envelope for EnvelopeSegment {
    fn volume(&self, cycle: u32) -> f32 {
        if cycle < self.start || cycle > self.end {
            0.0
        } else {
            let envelope_length = self.end - self.start;
            let since_start = cycle - self.start;
            let distance_through = since_start as f32 / envelope_length as f32;

            self.start_volume * (1.0 - distance_through) + self.end_volume * (distance_through)
        }
    }

    fn complete(&self, cycle: u32) -> bool {
        cycle > self.end
    }
}

struct Envelopes (Vec<EnvelopeSegment>);

impl Envelope for Envelopes {
    fn volume(&self, cycle: u32) -> f32 {
        let Envelopes(envelopes) = self;
        let mut volume = 0.0;
        for envelope in envelopes {
            volume += envelope.volume(cycle);
        }
        volume
    }

    fn complete(&self, cycle: u32) -> bool {
        let Envelopes(envelopes) = self;
        for envelope in envelopes {
            if !envelope.complete(cycle) {
                return false
            }
        }
        true
    }
}

#[derive(Clone, Debug)]
pub struct EnvSpec (Vec<(f32, f32)>);

impl EnvSpec {
    pub fn vols(points: Vec<(f32, f32)>) -> EnvSpec {
        EnvSpec(points)
    }

    fn for_frequency(&self, freq: i32) -> Envelopes {
        let EnvSpec(points) = self;
        let mut envelopes = Vec::new();
        for window in points.windows(2) {
            if let &[(start, start_volume), (end, end_volume)] = window {
                envelopes.push(EnvelopeSegment{
                    start: (freq as f32 * start) as u32, 
                    start_volume,
                    end: (freq as f32 * end) as u32,
                    end_volume
                });
            }
        }
        Envelopes(envelopes)
    }
}

pub fn initialise_audio(sdl_context: &sdl2::Sdl) -> Result<AudioDevice<AudioPlayer>, String> {
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),  // mono
        samples: Some(1024) 
    };

    let audio_device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        AudioPlayer {
            rng: SmallRng::from_entropy(),
            freq: spec.freq,
            cycles: 0,
            queue: BinaryHeap::new(),
            channel: [Channel::Silence {}, Channel::Silence{}, Channel::Silence {}, Channel::Silence{}]
        }
    }).unwrap();

    // Start playback
    audio_device.resume();
    Ok(audio_device)
}

pub fn play_tune(device: &mut AudioDevice<AudioPlayer>, PlayTune(channel, tune): &PlayTune) {
    device.lock().cue(*channel, tune);
}

struct Wave {
    phase_inc: f32,
    phase: f32,
    envelope: Envelopes,
    cycle: u32,
    waveform: Waveform,
}

struct Noise {
    up: bool,
    next_flip: u32,
    distribution: Uniform<u32>,
    envelope: Envelopes,
    cycle: u32,
}

enum Channel {
    Wave(Wave), 
    Silence, 
    Noise(Noise)
}

impl Channel {
    fn next_cycle(&mut self, rng: &mut SmallRng) {
        match self {
            Channel::Wave(wave) => {
                wave.phase = (wave.phase + wave.phase_inc) % 1.0;
                wave.cycle = wave.cycle + 1;
            },
            Channel::Silence => { },
            Channel::Noise(noise) => { 
                noise.up = if noise.next_flip == 0 { !noise.up } else { noise.up };
                noise.next_flip = if noise.next_flip == 0 { 
                    noise.distribution.sample(rng) 
                } else { 
                    noise.next_flip - 1 
                };
                noise.cycle = noise.cycle + 1;
            },
        }

    }
}

#[derive(Debug, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Cue {
    #[derivative(PartialOrd(compare_with="partial_reversed"), Ord(compare_with="reversed"))]
    start_at: u64,
    #[derivative(PartialEq="ignore", PartialOrd="ignore", Ord="ignore")]
    channel: usize,
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
    channel: [Channel;4]
}

impl AudioPlayer {
    fn set_channel(&mut self, channel: Channel, channel_no: usize) {
        self.channel[channel_no] = channel;
    }
}

impl AudioCallback for AudioPlayer {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        for x in out.iter_mut() {

            if let Some(note) = self.due() {
                self.play(&note);
            }

            *x = 0.0;

            for channel in &mut self.channel {
                match channel {
                    Channel::Wave(Wave { phase, envelope, waveform, cycle, .. }) => {
                        *x += waveform.amplitude(*phase) * envelope.volume(*cycle);
                    },
                    Channel::Silence {} => { },
                    Channel::Noise(Noise { up, envelope, cycle , .. }) => {
                        *x += if *up { envelope.volume(*cycle) } else { -envelope.volume(*cycle) }; 
                    }
                }
                channel.next_cycle(&mut self.rng);
            }
            self.cycles += 1;
        }
    } 
}

impl AudioPlayer {

    fn cue(&mut self, channel: usize, tune: &Vec<(Duration, Note)>) {
        self.queue.clear();
        for (delay, note) in tune {
            let cycles_before_start = (delay.as_secs_f64() * self.freq as f64) as u64;
            let start_at = cycles_before_start + self.cycles;
            self.queue.push(Cue { start_at, channel, note: note.clone() });
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

        
        let current_phase = match self.channel[0] {
            Channel::Wave(Wave { phase, ..}) => phase,
            _ => 0.0
        };
    
        let channel = match note {
            Note::Wave{ pitch, envelope , waveform } => {
                let envelope = envelope.for_frequency(freq);
                Channel::Wave(Wave {
                    phase_inc: pitch / freq as f32,
                    phase: current_phase,
                    envelope,
                    waveform: *waveform,
                    cycle: 0,
                })
            },
            Note::Noise{ low, high, envelope } => {
                let max_cycle = (freq as f32 / low) as u32;
                let min_cycle = (freq as f32 / high) as u32;
                let distribution : Uniform<u32> = Uniform::from(min_cycle..max_cycle);
                let envelope = envelope.for_frequency(freq);
                Channel::Noise(Noise {
                    up: false,
                    next_flip: 0,
                    distribution,
                    envelope,
                    cycle: 0,
                })
            },
            Note::Silence => Channel::Silence
        };
        self.set_channel(channel, 0);
    }
}

#[derive(Debug, Clone, Copy)]
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
    fn should_iterate_over_cues_in_increasing_time_order(){
        let mut heap = BinaryHeap::new();

        heap.push(Cue { start_at: 1, channel: 0, note: Note::Silence });
        heap.push(Cue { start_at: 2, channel: 0, note: Note::Silence });
        heap.push(Cue { start_at: 3, channel: 0, note: Note::Silence });
        heap.push(Cue { start_at: 4, channel: 0, note: Note::Silence });

        assert_eq!(heap.pop().unwrap().start_at, 1);
        assert_eq!(heap.pop().unwrap().start_at, 2);
        assert_eq!(heap.pop().unwrap().start_at, 3);
        assert_eq!(heap.pop().unwrap().start_at, 4);
        assert_eq!(heap.pop(), None);
    }

    #[test]
    fn should_smoothly_iterate_between_volumes() {
        let envelope = EnvelopeSegment { start: 0, start_volume: 0.0, end: 100, end_volume: 1.0 };
        assert_eq!(envelope.volume(0), 0.0);
        assert_eq!(envelope.volume(50), 0.5);
        assert_eq!(envelope.volume(100), 1.0);
    }

    #[test]
    fn should_return_0_volume_outside_range() {
        let envelope = EnvelopeSegment { start: 100, start_volume: 0.0, end: 200, end_volume: 1.0 };
        assert_eq!(envelope.volume(50), 0.0);
        assert_eq!(envelope.volume(250), 0.0);
    }

}