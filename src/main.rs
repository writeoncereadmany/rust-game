mod app;
mod entities;
mod shapes;
mod controller;
mod fps_counter;
mod game_loop;
mod graphics;
mod map;
mod world;

use sdl2::EventPump;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use entities::coin::Coin;
use entities::hero::Hero;
use controller::Controller;
use fps_counter::FpsCounter;
use game_loop::run_game_loop;
use graphics::lo_res_renderer::{ Layer, LoResRenderer };
use graphics::sprite::Sprite;
use graphics::map_renderer::render_map;
use graphics::text_renderer::SpriteFont;
use map::Map;
use app::app::App;
use app::assets::Assets;
use world::world::{Tile, World};
use world::stage::{border, stage1};


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
        .present_vsync()
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

    let controller = Controller::new(Keycode::Z, Keycode::X, Keycode::Semicolon, Keycode::Period, Keycode::RShift);

    let mut map_builder : Map<Tile> = Map::new(COLUMNS, ROWS, TILE_WIDTH, TILE_HEIGHT);
    border(&mut map_builder, Tile::STONE, COLUMNS, ROWS);
    stage1(&mut map_builder, Tile::STONE);
    let map = map_builder.add_edges();

    let coins: Vec<Coin> = vec![(1.0, 2.0)]
        .iter()
        .map(|(x, y)| (x * TILE_WIDTH as f64, y * TILE_WIDTH as f64))
        .map(|(x, y)| Coin::new(x, y, 12, 12, &assets))
        .collect();

    let tile = Sprite::new(&assets.tilesheet, Rect::new(0, 0, 12, 12));
    render_map(&map, &Layer::BACKGROUND, &mut renderer, | _t | { &tile });

    let spritefont = SpriteFont::new(&assets.spritefont, 8, 8);

    let ball = Hero::new(
        (TILE_WIDTH * COLUMNS as u32 / 2) as f64, 
        (TILE_HEIGHT * ROWS as u32 / 2) as f64, 
        12, 
        12, 
        &assets,
        controller
    );

    let world: World = World {
        ball,
        coins,
        map,
    };

    let mut app = App {
        game_controller_subsystem, 
        active_controller: None,
        spritefont,
        fps_counter: FpsCounter::new(),
        world
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut app, &mut renderer, &mut event_pump)?;

    Ok(())
}