mod app;
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
use shapes::push::Push;
use controller::Controller;
use fps_counter::FpsCounter;
use game_loop::{GameEvents, run_game_loop};
use graphics::lo_res_renderer::LoResRenderer;
use graphics::sprite::Sprite;
use graphics::renderer::Renderer;
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
const ACCEL: f64 = 200.0;
const VEL_CAP: f64 = 200.0;
const JUMP_SPEED: f64 = 200.0;
const WALLJUMP_DY: f64 = 180.0;
const WALLJUMP_DX: f64 = 150.0;
const WALL_STICK: f64 = 10.0;
const GRAVITY: f64 = 500.0;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum Layer {
    BACKGROUND,
    FOREGROUND
}

fn cap(val: f64, max: f64) -> f64 {
    if val > max {
        max
    } else if val < -max {
        -max
    } else {
        val
    }
}

impl <'a> GameEvents<'a, LoResRenderer<'a, Layer>> for App<'a> {
    fn update(&mut self, dt: Duration) -> Result<(), String> {

        self.world.ball.dx += self.controller.x() as f64 * ACCEL * dt.as_secs_f64();            
        self.world.ball.dx = cap(self.world.ball.dx, VEL_CAP);
        if self.controller.x() == 0 {
            match self.world.ball.last_push {
                (x, _) if x > 0.0 => {
                    self.world.ball.dx -= WALL_STICK;
                }
                (x, _) if x < 0.0 => {
                    self.world.ball.dx += WALL_STICK;
                }
                _ => {}
            }
        }
        self.world.ball.x += self.world.ball.dx * dt.as_secs_f64();
        
        if self.controller.jump() {
            match self.world.ball.last_push {
                (_, y) if y > 0.0 => { self.world.ball.dy = JUMP_SPEED; },
                (x, _) if x > 0.0 => { 
                    self.world.ball.dy = WALLJUMP_DY;
                    self.world.ball.dx = WALLJUMP_DX;
                },
                (x, _) if x < 0.0 => {
                    self.world.ball.dy = WALLJUMP_DY;
                    self.world.ball.dx = -WALLJUMP_DX;
                }
                _ => {} 
            }
        }
        self.world.ball.dy -= GRAVITY * dt.as_secs_f64();
        self.world.ball.dy = cap(self.world.ball.dy, VEL_CAP);

        self.world.ball.y += self.world.ball.dy * dt.as_secs_f64();

        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        for (_pos, t) in self.world.map.overlapping(&self.world.ball.mesh().bbox()) {
            let push = t.mesh.push(&self.world.ball.mesh());
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 && x.signum() == -self.world.ball.dx.signum() {
                        self.world.ball.x += x;
                        tot_x_push += x;
                        self.world.ball.dx = 0.0;
                    }
                    if y != 0.0 && y.signum() == -self.world.ball.dy.signum() {
                        self.world.ball.y += y;
                        tot_y_push += y;
                        self.world.ball.dy = 0.0;
                    }
                }
            }
        }
        self.world.ball.last_push = (tot_x_push, tot_y_push);
        
        Ok(())
    }

    fn render(&mut self, renderer: &mut LoResRenderer<'a, Layer>) -> Result<(), String> {
        self.fps_counter.on_frame();

        renderer.clear(&Layer::FOREGROUND).unwrap();

        renderer.draw(&Layer::FOREGROUND, &self.world.ball.sprite, self.world.ball.x as i32, self.world.ball.y as i32);

        self.spritefont.render(self.fps_counter.fps().to_string() + " fps", 2, 2, renderer, &Layer::FOREGROUND);
      
        renderer.present()?;

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.controller.on_event(event);
        match event {
            Event::Quit {..} => return Err("Escape pressed: ending game".into()),
            Event::KeyDown { keycode: Some(Keycode::Escape), ..} => return Err("Esc pressed: ending game".into()),
            Event::ControllerDeviceAdded{ which, .. } => { 
                self.active_controller = self.game_controller_subsystem.open(*which).ok(); 
            }
            _ => {}
        }
        Ok(())
    }
}

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

    let tile = Sprite::new(&assets.tilesheet, Rect::new(0, 0, 12, 12));
    render_map(&map, &Layer::BACKGROUND, &mut renderer, | _t | { &tile });

    let spritefont = SpriteFont::new(&assets.spritefont, 8, 8);

    let ball = Ball::new(
        (TILE_WIDTH * COLUMNS as u32 / 2) as f64, 
        (TILE_HEIGHT * ROWS as u32 / 2) as f64, 
        12, 
        12, 
        &assets);

    let world: World = World {
        ball,
        map,
    };

    let mut app = App {
        game_controller_subsystem, 
        active_controller: None,
        spritefont,
        controller,
        fps_counter: FpsCounter::new(),
        world
    };

    let mut event_pump: EventPump = sdl_context.event_pump()?;

    run_game_loop(&mut app, &mut renderer, &mut event_pump)?;

    Ok(())
}