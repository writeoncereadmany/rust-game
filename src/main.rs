mod fps_counter;
mod game_loop;
mod lo_res_renderer;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::EventPump;
use sdl2::event::{Event};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;

use fps_counter::FpsCounter;
use game_loop::{Game, run_game_loop};
use lo_res_renderer::LoResRenderer;


const COLUMNS: u32 = 32;
const ROWS: u32 = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

struct TileSplatter<'a> {
    ball: Texture<'a>,
    numbers: Vec<Texture<'a>>,
    ball_x: f64,
    ball_y: f64,
    fps_counter: FpsCounter
}

impl Game for TileSplatter<'_> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.draw(|c| {
            render_number(18, 2, self.fps_counter.fps(), c, &self.numbers).unwrap();
            c.copy(&self.ball, None, Rect::new(self.ball_x as i32, self.ball_y as i32, 12, 12)).unwrap();
        }).unwrap();

        renderer.present()?;

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        match event {
            Event::Quit {..} => return Err("Escape pressed: ending game".into()),
            Event::KeyDown { keycode: Some(key_pressed), .. } => {
                match key_pressed {
                    Keycode::Escape => return Err("Escape pressed: ending game".into()),
                    Keycode::Z => self.ball_x = self.ball_x - 10.0,
                    Keycode::X => self.ball_x = self.ball_x + 10.0,
                    Keycode::P => self.ball_y = self.ball_y - 10.0,
                    Keycode::L => self.ball_y = self.ball_y + 10.0,
                    _ => {}
                }
            },
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

    let (width, height) = window.size();
    
    print!("Screen resolution: {}x{}", width, height);

    let canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");


    let texture_creator = canvas.texture_creator();

    let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();
    let tile = texture_creator.load_texture(assets.join("12x12tile.png"))?;

    let mut renderer = LoResRenderer::new(canvas, &texture_creator, TILE_WIDTH * COLUMNS, TILE_HEIGHT * ROWS).unwrap();

    renderer.draw_to_background(|c| {
        c.set_draw_color(Color::BLACK);
        c.clear();

        for x in 0..COLUMNS {
            c.copy(&tile, None, Rect::new((x * TILE_WIDTH) as i32, 0, TILE_WIDTH, TILE_HEIGHT)).unwrap();
            c.copy(&tile, None, Rect::new((x * TILE_WIDTH) as i32, ((ROWS - 1) * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT)).unwrap();
        }
        for y in 0..ROWS {
            c.copy(&tile, None, Rect::new(0, (y * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT)).unwrap();
            c.copy(&tile, None, Rect::new(((COLUMNS - 1) * TILE_WIDTH) as i32, (y * TILE_HEIGHT) as i32, TILE_WIDTH, TILE_HEIGHT)).unwrap();
        }

    }).unwrap();

    let numbers : Result<Vec<Texture>, String> = (0..10).map(|n| { 
        let number = assets.join(n.to_string() + ".png");
        texture_creator.load_texture(number)
    }).collect();
    let numbers = numbers?;

    let mut splatto: TileSplatter = TileSplatter {
        ball: texture_creator.load_texture(assets.join("ball.png"))?,
        numbers,
        ball_x: (TILE_WIDTH * COLUMNS / 2) as f64,
        ball_y: (TILE_HEIGHT * ROWS / 2) as f64,
        fps_counter: FpsCounter::new()
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut splatto, &mut renderer, &mut event_pump)?;

    Ok(())
}

fn render_number(x: i32, y: i32, num: usize, canvas: &mut Canvas<Window>, numbers : &Vec<Texture>) -> Result<(), String> {
    let mut digit = num % 10;
    let mut remainder = num / 10;
    let mut offset = 0;

    while digit > 0 || remainder > 0
    {
        canvas.copy(&numbers.get(digit).unwrap(), None, Rect::new(x - offset, y, 8, 8))?;
        
        offset += 8;
        digit = remainder % 10;
        remainder = remainder / 10;
    }
    Ok(())
}