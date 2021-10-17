mod fps_counter;
mod game_loop;

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


const COLUMNS: u32 = 32;
const ROWS: u32 = 18;
const TILE_WIDTH: u32 = 12;
const TILE_HEIGHT: u32 = 12;

struct TileSplatter<'a> {
    tile: Texture<'a>,
    ball: Texture<'a>,
    numbers: Vec<Texture<'a>>,
    x_offset: i32,
    y_offset: i32,
    ball_x: f64,
    ball_y: f64,
    scale: f64,
    tiles: Vec<(f64, f64)>,
    fps_counter: FpsCounter
}

impl Game for TileSplatter<'_> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        Ok(())
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.fps_counter.on_frame();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for &(x, y) in &self.tiles {
            render_image(x, y, self.scale, self.x_offset, self.y_offset, canvas, &self.tile)?;
        }
        render_number(40, 6, self.fps_counter.fps(), canvas, &self.numbers)?;
        render_image(self.ball_x, self.ball_y, self.scale, self.x_offset, self.y_offset, canvas, &self.ball)?;

        canvas.present();
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

    let x_scale = width as f64 / (COLUMNS * TILE_WIDTH) as f64;
    let y_scale = height as f64 / (ROWS * TILE_HEIGHT) as f64;
    let scale = f64::min(x_scale, y_scale);

    let x_offset = (width as i32 - ((COLUMNS * TILE_WIDTH) as f64 * scale) as i32) / 2;
    let y_offset = (height as i32 - ((ROWS * TILE_HEIGHT) as f64 * scale) as i32) / 2;
    
    print!("Screen resolution: {}x{}", width, height);

    let mut canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");

    let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

    let texture_creator = canvas.texture_creator();

    let numbers : Result<Vec<Texture>, String> = (0..10).map(|n| { 
        let number = assets.join(n.to_string() + ".png");
        texture_creator.load_texture(number)
    }).collect();
    let numbers = numbers?;

    let mut tiles = Vec::new();
    for x in 0..COLUMNS
    {
        tiles.push((((x * TILE_WIDTH) as f64), 0.0));
        tiles.push((((x * TILE_WIDTH) as f64), ((ROWS - 1) * TILE_HEIGHT) as f64));
    }
    for y in 1..(ROWS - 1)
    {
        tiles.push((0.0, ((y * TILE_HEIGHT) as f64)));
        tiles.push((((COLUMNS - 1) * TILE_WIDTH) as f64, (y * TILE_HEIGHT) as f64));
    }

    let mut splatto: TileSplatter = TileSplatter {
        tile: texture_creator.load_texture(assets.join("12x12tile.png"))?,
        ball: texture_creator.load_texture(assets.join("ball.png"))?,
        numbers,
        x_offset,
        y_offset,
        ball_x: (TILE_WIDTH * COLUMNS / 2) as f64,
        ball_y: (TILE_HEIGHT * ROWS / 2) as f64,
        scale,
        tiles,
        fps_counter: FpsCounter::new()
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut splatto, &mut canvas, &mut event_pump)?;

    Ok(())
}

fn render_number(x: i32, y: i32, num: usize, canvas: &mut Canvas<Window>, numbers : &Vec<Texture>) -> Result<(), String> {
    let mut digit = num % 10;
    let mut remainder = num / 10;
    let mut offset = 0;

    while digit > 0 || remainder > 0
    {
        canvas.copy(&numbers.get(digit).unwrap(), None, Rect::new(x - offset, y, 16, 16))?;
        
        offset += 16;
        digit = remainder % 10;
        remainder = remainder / 10;
    }
    Ok(())
}

fn render_image(x: f64, y: f64, scale: f64, x_offset: i32, y_offset: i32, canvas: &mut Canvas<Window>, tile: &Texture) -> Result<(), String> {
    canvas.copy(tile, None, Rect::new((x * scale) as i32 + x_offset, (y * scale) as i32 + y_offset, (TILE_WIDTH as f64 * scale) as u32, (TILE_HEIGHT as f64 * scale) as u32))?;
    Ok(())
}