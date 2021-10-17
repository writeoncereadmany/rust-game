mod fps_counter;
mod game_loop;

use rand::Rng;
use rand::prelude::ThreadRng;
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

struct TileSplatter<'a> {
    tile: Texture<'a>,
    numbers: Vec<Texture<'a>>,
    width: u32,
    height: u32,
    tiles: Vec<(f64, f64)>,
    fps_counter: FpsCounter,
    rng: ThreadRng
}

impl Game for TileSplatter<'_> {
    fn update(&mut self, _delta: Duration) -> Result<(), String> {
        self.tiles.push((self.rng.gen_range(0.0..self.width as f64), self.rng.gen_range(0.0..self.height as f64)));
        Ok(())
    }

    fn render(&mut self, canvas: &mut Canvas<Window>) -> Result<(), String> {
        self.fps_counter.on_frame();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        for &(x, y) in &self.tiles {
            render_tile(x, y, canvas, &self.tile)?;
        }
        render_number(40, 6, self.fps_counter.fps(), canvas, &self.numbers)?;
        render_number(720, 32, self.tiles.len(), canvas, &self.numbers)?;

        canvas.present();
        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Err("Escape pressed: ending game".into());
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

    let mut canvas : Canvas<Window> = window
        .into_canvas()
        .present_vsync()
        .build()
        .expect("could not make a canvas");

    let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

    let texture_creator = canvas.texture_creator();
    let tile : Texture = texture_creator.load_texture("assets/12x12tile.png")?;

    let numbers : Result<Vec<Texture>, String> = (0..10).map(|n| { 
        let number = assets.join(n.to_string() + ".png");
        let number = number.to_str().ok_or(format!("Could not load {}.png", n))?;
        texture_creator.load_texture(number)
    }).collect();
    let numbers = numbers?;

    let mut splatto: TileSplatter = TileSplatter {
        tile,
        numbers,
        width,
        height,
        tiles: Vec::new(),
        fps_counter: FpsCounter::new(),
        rng: rand::thread_rng()
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

fn render_tile(x: f64, y: f64, canvas: &mut Canvas<Window>, tile: &Texture) -> Result<(), String> {
    canvas.copy(tile, None, Rect::new(x as i32, y as i32, 24, 24))?;
    Ok(())
}