mod controller;
mod fps_counter;
mod game_loop;
mod lo_res_renderer;
mod sprite;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Canvas};
use sdl2::video::Window;

use controller::Controller;
use fps_counter::FpsCounter;
use game_loop::{Game, run_game_loop};
use lo_res_renderer::LoResRenderer;
use sprite::Sprite;

const COLUMNS: u32 = 32;
const ROWS: u32 = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

struct TileSplatter<'a> {
    ball_sprite: Sprite<'a>,
    numbers: Vec<Sprite<'a>>,
    controller: Controller,
    ball_x: f64,
    ball_y: f64,
    fps_counter: FpsCounter
}

impl Game for TileSplatter<'_> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        self.ball_x += self.controller.x() as f64;
        self.ball_y += self.controller.y() as f64;
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.draw(|c| {
            render_number(18, 2, self.fps_counter.fps(), c, &self.numbers).unwrap();
            self.ball_sprite.draw_to(c, self.ball_x as i32, self.ball_y as i32).unwrap();
        }).unwrap();

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

    let mut renderer = LoResRenderer::new(canvas, &texture_creator, TILE_WIDTH * COLUMNS, TILE_HEIGHT * ROWS).unwrap();

    let controller = Controller::new(Keycode::Z, Keycode::X, Keycode::Semicolon, Keycode::Period);

    renderer.draw_to_background(|c| {
        c.set_draw_color(Color::BLACK);
        c.clear();

        for x in 0..COLUMNS {
            tile.draw_to(c, (x * TILE_WIDTH) as i32, 0).unwrap();
            tile.draw_to(c, (x * TILE_WIDTH) as i32, ((ROWS - 1) * TILE_HEIGHT) as i32).unwrap();
        }
        for y in 0..ROWS {
            tile.draw_to(c, 0, (y * TILE_HEIGHT) as i32).unwrap();
            tile.draw_to(c, ((COLUMNS-1) * TILE_WIDTH) as i32, (y * TILE_HEIGHT) as i32).unwrap();
        }
    }).unwrap();

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
        ball_x: (TILE_WIDTH * COLUMNS / 2) as f64,
        ball_y: (TILE_HEIGHT * ROWS / 2) as f64,
        fps_counter: FpsCounter::new()
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut splatto, &mut renderer, &mut event_pump)?;

    Ok(())
}

fn render_number(x: i32, y: i32, num: usize, canvas: &mut Canvas<Window>, numbers : &Vec<Sprite>) -> Result<(), String> {
    let mut digit = num % 10;
    let mut remainder = num / 10;
    let mut offset = 0;

    while digit > 0 || remainder > 0
    {
        numbers.get(digit).unwrap().draw_to(canvas, x - offset, y)?;
        
        offset += 8;
        digit = remainder % 10;
        remainder = remainder / 10;
    }
    Ok(())
}