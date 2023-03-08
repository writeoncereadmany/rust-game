mod app;
mod audio;
mod entities;
mod shapes;
mod controller;
mod fps_counter;
mod game_loop;
mod graphics;
mod map;
mod music;
mod game;
mod world;
mod sign;
mod events;
mod screens;

use screens::title::Title;
use sdl2::EventPump;
use sdl2::image::{self, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use audio::audio::initialise_audio;
use game_loop::run_game_loop;
use graphics::renderer::{Renderer};
use graphics::sprite::SpriteSheet;
use app::app::App;
use app::assets::Assets;
use events::Events;
use game::game::Game;
use screens::screens::Screen;

const COLUMNS: usize = 26;
const ROWS: usize = 15;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let audio_device = initialise_audio(&sdl_context)?;

    image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("rust-sdl2 demo", 1280, 720)
        .fullscreen()
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

    let mut events = Events::new();

    let app = App {
        game_controller_subsystem, 
        audio_device,
        active_controller: None,
        assets: &assets,
        screen: Screen::TitleScreen(Title)
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    run_game_loop(app, &mut renderer, &mut event_pump, 1, events)?;

    Ok(())
}