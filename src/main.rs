mod collisions;
mod controller;
mod fps_counter;
mod game_loop;
mod lo_res_renderer;
mod map;

use std::time::Duration;

use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use collisions::{ConvexMesh, Push};
use controller::Controller;
use fps_counter::FpsCounter;
use game_loop::{Game, run_game_loop};
use lo_res_renderer::{LoResRenderer, Sprite};
use map::Map;


const COLUMNS: usize = 32;
const ROWS: usize = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

struct TileSplatter<'a> {
    ball_sprite: Sprite<'a>,
    numbers: Vec<Sprite<'a>>,
    controller: Controller,
    map: Map<ColTile>,
    ball_x: f64,
    ball_y: f64,
    fps_counter: FpsCounter
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Layer {
    BACKGROUND,
    FOREGROUND
}

#[derive(Clone)]
enum Tile {
    STONE
}

#[derive(Clone)]
struct ColTile {
    tile: Tile,
    mesh: ConvexMesh
}

impl <'a> Game<'a, Layer> for TileSplatter<'a> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        self.ball_x += self.controller.x() as f64;
        self.ball_y += self.controller.y() as f64;
        for (_x, _y, t) in &self.map {
            let ball_rect = ConvexMesh::rect(self.ball_x, self.ball_y, 12.0, 12.0);
            if t.mesh.aabbs_overlap(&ball_rect)
            {
                match t.mesh.push(&ball_rect) {
                    None => {},
                    Some((x, y)) => {
                        self.ball_x += x;
                        self.ball_y += y;
                    }
                }
            }
        }
        
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.clear(&Layer::FOREGROUND).unwrap();

        renderer.draw(&Layer::FOREGROUND, &self.ball_sprite, self.ball_x as i32, self.ball_y as i32);
        render_number(10, ((TILE_HEIGHT * ROWS as u32) - 10) as i32, self.fps_counter.fps(), renderer, &self.numbers).unwrap();
        
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

    let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

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

    let tile = texture_creator.load_texture(assets.join("12x12tile.png"))?;
    let tile = Sprite::new(&tile, Rect::new(0, 0, 12, 12));

    let mut renderer = LoResRenderer::new(
        canvas, 
        &texture_creator, 
        TILE_WIDTH * COLUMNS as u32, 
        TILE_HEIGHT * ROWS as u32, 
        vec!(Layer::BACKGROUND, Layer::FOREGROUND)
    ).unwrap();

    let controller = Controller::new(Keycode::Z, Keycode::X, Keycode::Semicolon, Keycode::Period);

    renderer.clear(&Layer::BACKGROUND).unwrap();

    let mut map_builder : Map<Tile> = Map::new(COLUMNS, ROWS);

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

    let mut map : Map<ColTile> = Map::new(COLUMNS, ROWS);

    map_builder.into_iter().for_each(|(x, y, tile)| {
        let left = (TILE_WIDTH * (x as u32)) as f64;
        let right = (TILE_WIDTH * ((x + 1) as u32)) as f64;
        let top = (TILE_HEIGHT * ((y + 1) as u32)) as f64;
        let bottom = (TILE_HEIGHT * (y as u32)) as f64;

        let points = vec![(left, bottom), (left, top), (right, top), (right, bottom)];

        let mut normals : Vec<(f64, f64)> = Vec::new();

        if map_builder.get(x-1, y).is_none() { normals.push((-1.0, 0.0)); }
        if map_builder.get(x + 1, y).is_none() { normals.push((1.0, 0.0)); }
        if map_builder.get(x, y - 1).is_none() { normals.push((0.0, -1.0)); }
        if map_builder.get(x, y + 1).is_none() { normals.push((0.0, 1.0)); }

        let mesh = ConvexMesh::new(points, normals);
        map.put(x, y, ColTile { tile, mesh });
    });

    for (x, y, _t) in &map {
        renderer.draw(&Layer::BACKGROUND, &tile, (x as u32 * TILE_WIDTH) as i32, (y as u32 * TILE_HEIGHT) as i32)
    }

    let numbers_spritesheet = texture_creator.load_texture(assets.join("numbers.png"))?;
    let numbers: Vec<Sprite<'_>> = (0..10).map(|n| {
        Sprite::new(&numbers_spritesheet, Rect::new(n*8, 0, 8, 8))
    }).collect();

    let ball_tex = texture_creator.load_texture(assets.join("ball.png"))?;
    let ball_sprite = Sprite::new(&ball_tex, Rect::new(0, 0, 12, 12));

    let mut splatto: TileSplatter = TileSplatter {
        ball_sprite,
        numbers,
        controller,
        map,
        ball_x: (TILE_WIDTH * COLUMNS as u32 / 2) as f64,
        ball_y: (TILE_HEIGHT * ROWS as u32 / 2) as f64,
        fps_counter: FpsCounter::new()
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut splatto, &mut renderer, &mut event_pump)?;

    Ok(())
}

fn render_number<'a>(x: i32, y: i32, num: usize, renderer: &mut LoResRenderer<'a, Layer>, numbers : &Vec<Sprite<'a>>) 
-> Result<(), String> {
    let mut digit = num % 10;
    let mut remainder = num / 10;
    let mut offset = 0;

    while digit > 0 || remainder > 0
    {
        let num_sprite = numbers.get(digit).unwrap();
        renderer.draw(&Layer::FOREGROUND, num_sprite, x - offset, y);
        
        offset += 8;
        digit = remainder % 10;
        remainder = remainder / 10;
    }
    Ok(())
}