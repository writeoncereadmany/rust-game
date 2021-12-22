mod entities;
mod shapes;
mod controller;
mod fps_counter;
mod game_loop;
mod graphics;
mod map;
mod world;

use std::time::Duration;

use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use entities::ball::Ball;
use shapes::vec2d::Vec2d;
use shapes::push::Push;
use controller::Controller;
use fps_counter::FpsCounter;
use game_loop::{Game, run_game_loop};
use graphics::lo_res_renderer::LoResRenderer;
use graphics::sprite::Sprite;
use graphics::renderer::Renderer;
use graphics::map_renderer::render_map;
use graphics::text_renderer::SpriteFont;
use map::Map;
use world::assets::Assets;
use world::world::{Tile, World};


const COLUMNS: usize = 32;
const ROWS: usize = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Layer {
    BACKGROUND,
    FOREGROUND
}

impl <'a> Game<'a, LoResRenderer<'a, Layer>> for World<'a> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        self.ball.x += self.controller.x() as f64;
        self.ball.y += self.controller.y() as f64;
        for (_pos, t) in self.map.overlapping(&self.ball.mesh().bbox()) {
            match t.mesh.push(&self.ball.mesh()) {
                None => {},
                Some((x, y)) => {
                    if (x, y).sq_len() < 100.0 {
                        self.ball.x += x;
                        self.ball.y += y;
                    }
                }
            }
        }
        
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.clear(&Layer::FOREGROUND).unwrap();

        renderer.draw(&Layer::FOREGROUND, &self.ball.sprite, self.ball.x as i32, self.ball.y as i32);

        self.spritefont.render(self.fps_counter.fps().to_string() + "fps", 2, 2, renderer, &Layer::FOREGROUND);
        
        renderer.present()?;

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.controller.on_event(event);
        match event {
            Event::Quit {..} => return Err("Escape pressed: ending game".into()),
            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
            _ => {}
        }
        Ok(())
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
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

    let assets = Assets::new(&texture_creator)?;

    let mut renderer = LoResRenderer::new(
        canvas, 
        &texture_creator, 
        TILE_WIDTH * COLUMNS as u32, 
        TILE_HEIGHT * ROWS as u32, 
        vec!(Layer::BACKGROUND, Layer::FOREGROUND)
    ).unwrap();

    let controller = Controller::new(Keycode::Z, Keycode::X, Keycode::Semicolon, Keycode::Period);

    let mut map_builder : Map<Tile> = Map::new(COLUMNS, ROWS, TILE_WIDTH, TILE_HEIGHT);

    map_builder.row(0, 0, COLUMNS, Tile::STONE)
       .row(0, ROWS - 1, COLUMNS, Tile::STONE)
       .column(0, 0, ROWS, Tile::STONE)
       .column(COLUMNS - 1, 0, ROWS, Tile::STONE);
    
    map_builder.row(4, 4, 4, Tile::STONE)
       .row(24, 4, 4, Tile::STONE)
       .row(1, 8, 5, Tile::STONE)
       .row(10, 6, 12, Tile::STONE)
       .row(4, 12, 6, Tile::STONE)
       .row(26, 8, 5, Tile::STONE)
       .row(22, 12, 6, Tile::STONE)
       .column(10, 6, 7, Tile::STONE)
       .column(21, 6, 7, Tile::STONE)
       .column(15, 10, 8, Tile::STONE)
       .column(16, 10, 8, Tile::STONE)
       ;

    let map = map_builder.add_edges();

    let tile = Sprite::new(&assets.tilesheet, Rect::new(0, 0, 12, 12));

    render_map(&map, &Layer::BACKGROUND, &mut renderer, | _t | { &tile });

    let numbers: Vec<Sprite<'_>> = (0..10).map(|n| {
        Sprite::new(&assets.numbersheet, Rect::new(n*8, 0, 8, 8))
    }).collect();

    let spritefont = SpriteFont::new(&assets.spritefont, 8, 8);

    let ball = Ball::new(
        (TILE_WIDTH * COLUMNS as u32 / 2) as f64, 
        (TILE_HEIGHT * ROWS as u32 / 2) as f64, 
        12, 
        12, 
        &assets);

    let mut world: World = World {
        ball,
        spritefont,
        controller,
        map,
        fps_counter: FpsCounter::new()
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut world, &mut renderer, &mut event_pump)?;

    Ok(())
}