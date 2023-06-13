mod app;
mod entities;
mod controller;
mod game_loop;
mod map;
mod music;
mod game;
mod world;
mod sign;
mod screens;

use controller::Controller;
use screens::title::Title;
use sdl2::EventPump;
use sdl2::image::{self, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::render::{Canvas};
use sdl2::video::Window;

use engine::events::Events;
use engine::audio::audio::initialise_audio;

use game_loop::run_game_loop;
use engine::graphics::renderer::Renderer;
use engine::graphics::sprite::SpriteSheet;
use app::app::App;
use app::assets::Assets;
use screens::screens::Screen;

const COLUMNS: usize = 30;
const ROWS: usize = 20;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
    let audio_device = initialise_audio(&sdl_context)?;

    image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("rust-sdl2 demo", 0, 0)
        .fullscreen_desktop()
        .build()
        .expect("could not initialize video subsystem");

    video_subsystem.text_input().start();

    let canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
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

    let events = Events::new();

    let app = App {
        video_subsystem,
        game_controller_subsystem, 
        audio_device,
        active_controller: None,
        controller: Controller::new(Keycode::Z, Keycode::X, Keycode::RShift),
        assets: &assets,
        screen: Screen::TitleScreen(Title),
        scores: App::starting_scores(),
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;
    run_game_loop(app, &mut renderer, &mut event_pump, 1, events)?;

    Ok(())
}
