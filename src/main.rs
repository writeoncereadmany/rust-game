mod app;
mod entities;
mod shapes;
mod controller;
mod fps_counter;
mod game_loop;
mod graphics;
mod map;
mod game;
mod world;
mod sign;
mod timebuffer;

use std::time::Duration;

use sdl2::EventPump;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas};
use sdl2::video::Window;
use sdl2::audio::{AudioDevice, AudioSpecDesired, AudioCallback};

use fps_counter::FpsCounter;
use game_loop::run_game_loop;
use graphics::renderer::{Renderer};
use graphics::sprite::SpriteSheet;
use controller::Controller;
use app::app::App;
use app::assets::Assets;
use world::world::World;
use game::game::Game;

const COLUMNS: usize = 32;
const ROWS: usize = 18;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let audio_subsystem = sdl_context.audio()?;

    let desired_spec = AudioSpecDesired {
        freq: Some(44100),
        channels: Some(1),  // mono
        samples: None       // default sample size
    };

    let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
        // initialize the audio callback
        TwoChannels {
            freq: spec.freq as f32,
            channel1: Sawtooth {
                pitch: 440.0,
                phase: 0.0,
                volume: 0.0
            },
            channel2: Sawtooth {
                pitch: 256.0,
                phase: 0.0,
                volume: 0.0               
            }
        }
    }).unwrap();

    // Start playback
    device.resume();

    image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .fullscreen_desktop()
        .build()
        .expect("could not initialize video subsystem");

    let canvas : Canvas<Window> = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();

    let assets = Assets::new(&texture_creator)?;

    let spritesheet = SpriteSheet::new(&assets.spritesheet, 12, 12);

    let spritefont = SpriteSheet::new(&assets.spritefont, 8, 8);

    let mut renderer = Renderer::new(
        canvas,
        &texture_creator,
        spritesheet,
        spritefont,
        COLUMNS as u32,
        ROWS as u32,
    ).unwrap(); 

    let world: World = World::new(&assets.levels[0], Controller::new(Keycode::Z, Keycode::X, Keycode::RShift));
    let game: Game = Game{ world, levels: &assets.levels, level: 0, score: 0 };

    let app = App {
        game_controller_subsystem, 
        active_controller: None,
        fps_counter: FpsCounter::new(),
        game
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(app, &mut renderer, &mut event_pump, 1)?;

    Ok(())
}

struct TwoChannels {
    freq: f32,
    channel1: Sawtooth,
    channel2: Sawtooth
}

impl AudioCallback for TwoChannels {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = self.channel1.volume(self.freq) + self.channel2.volume(self.freq);
        }
    }
}

struct Sawtooth {
    pitch: f32,
    phase: f32,
    volume: f32
}

impl Sawtooth {
    fn volume(&mut self, freq: f32) -> f32 {
        // Generate a square wave
        self.phase = (self.phase + (self.pitch / freq)) % 1.0;
        self.phase * self.volume
    }
}