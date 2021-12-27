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

use sdl2::EventPump;
use sdl2::image::{self, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use fps_counter::FpsCounter;
use game_loop::run_game_loop;
use graphics::lo_res_renderer::{ Layer, LoResRenderer };
use graphics::map_renderer::{render_map};
use graphics::renderer::Renderer;
use app::app::App;
use app::assets::Assets;
use world::world::World;
use game::game::Game;

const COLUMNS: usize = 32;
const ROWS: usize = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let game_controller_subsystem = sdl_context.game_controller()?;
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

    let mut renderer = LoResRenderer::new(
        canvas, 
        &texture_creator, 
        TILE_WIDTH * COLUMNS as u32, 
        TILE_HEIGHT * ROWS as u32, 
        vec!(Layer::BACKGROUND, Layer::FOREGROUND)
    ).unwrap();

    let assets = Assets::new(&texture_creator)?;

    let world: World = World::new(&assets, 0);

    let tile = assets.sprite(0, 1);
    render_map(&world.map, &Layer::BACKGROUND, &mut renderer, | _t | { &tile });

    let timebox = &assets.multisprite(2, 0, 2, 1);
    renderer.draw(&Layer::BACKGROUND, &timebox, TILE_WIDTH as i32 * 15, TILE_HEIGHT as i32 * (ROWS as i32- 1));

    let spritefont = &assets.spritefont();
    let game: Game = Game{ world, assets: &assets, level: 0, spritefont, score: 0 };

    let mut app = App {
        game_controller_subsystem, 
        active_controller: None,
        spritefont,
        fps_counter: FpsCounter::new(),
        game
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut app, &mut renderer, &mut event_pump)?;

    Ok(())
}