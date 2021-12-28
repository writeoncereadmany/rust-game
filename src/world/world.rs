use std::time::Duration;

use image::{ Rgb, RgbImage };

use crate::app::events::*;
use crate::shapes::push::Push;
use crate::entities::coin::Coin;
use crate::entities::hero::Hero;
use crate::entities::door::Door;
use crate::map::Map;
use crate::controller::Controller;
use crate::shapes::convex_mesh::Meshed;
use crate::game_loop::GameLoop;
use crate::graphics::renderer::{ Layer, Renderer, Justification, Tiled };

#[derive(Clone)]
pub enum Tile {
    STONE((i32, i32))
}

impl Tiled for Tile {
    fn tile(&self) -> (i32, i32) {
        match self {
            Tile::STONE(tile) => *tile
        }
    }
}

pub struct World {
    pub hero: Hero,
    pub coins: Vec<Coin>,
    pub doors: Vec<Door>,
    pub map: Map<Meshed<Tile>>,
    pub time: f64,
}

impl World {

    pub fn new(image: &RgbImage, tile_width: u32, tile_height: u32, controller: Controller) -> Self {
        let width = image.width();
        let height = image.height();
        let mut map : Map<Tile> = Map::new(width as usize, height as usize, tile_width, tile_height);
        let mut coins: Vec<Coin> = Vec::new();
        let mut hero: Option<Hero> = None;
        let mut doors: Vec<Door> = Vec::new();

        let mut id = 0;

        for x in 0..image.width() {
            for y in 0..height {
                let pixel: &Rgb<u8> = image.get_pixel(x, height - 1 - y);
                match pixel {
                    Rgb([255, 255, 255]) => { map.put(x as i32, y as i32, Tile::STONE((0, 1))); },
                    Rgb([255, 255, 0]) => { 
                        coins.push(Coin::new((x * tile_width) as f64, (y * tile_height) as f64, tile_width, tile_height, id));
                        id += 1;
                    },
                    Rgb([255, 0, 0]) => { doors.push(Door::new((x * tile_width) as f64, (y * tile_height) as f64, tile_width, tile_height))},
                    Rgb([0, 255, 0]) => { match hero {
                        None => { hero = Some(Hero::new(
                            (x * tile_width) as f64, 
                            (y * tile_height) as f64, 
                            tile_width, 
                            tile_height,
                            controller)); }
                        Some(_) => { panic!("Multiple hero start positions defined"); }
                    }},
                    _ => { }
                }
                
            }
        }

        let map = map.add_edges();

        World {
            hero: hero.unwrap(),
            map,
            coins,
            doors,
            time: 100.0
        }
    }
}

impl <'a> GameLoop<'a, Renderer<'a, Layer>, GEvent> for World {
    
    fn render(&self, renderer: &mut Renderer<'a, Layer>) -> Result <(), String> {
        renderer.draw_map(&self.map, &Layer::BACKGROUND);

        renderer.draw_multitile(&Layer::BACKGROUND, (2, 0), (2, 1), 15*12, 17*12);

        for coin in &self.coins {
            coin.render(renderer)?;
        }

        for door in &self.doors {
            door.render(renderer)?;
        }

        self.hero.render(renderer)?;

        renderer.draw_text(time_units(self.time), &Layer::FOREGROUND, 12*16, 12 * 17 + 2, Justification::CENTER);

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        self.hero.event(event, events)?;
        for coin in self.coins.iter_mut() {
            coin.event(event, events)?;
        }

        match event {
            Event::Time(dt) => { update(self, dt, events)?; },
            Event::Cleanup => { self.coins.retain(|coin| !coin.collected); }
            _ => { },
        }

        Ok(())
    }
}

fn update<'a>(world: &mut World, dt: &Duration, events: &mut Events) -> Result<(), String> {
        
    let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
    for (_pos, t) in world.map.overlapping(&world.hero.mesh().bbox()) {
        let push = t.mesh.push(&world.hero.mesh());
        match push {
            None => {},
            Some((x, y)) => {
                if x != 0.0 && world.hero.dx != 0.0 && x.signum() == -world.hero.dx.signum() {
                    world.hero.x += x;
                    tot_x_push += x;
                    world.hero.dx = 0.0;
                }
                if y != 0.0 && world.hero.dy != 0.0 && y.signum() == -world.hero.dy.signum() {
                    world.hero.y += y;
                    tot_y_push += y;
                    world.hero.dy = 0.0;
                }
            }
        }
    }
    world.hero.last_push = (tot_x_push, tot_y_push);

    let ball_mesh = world.hero.mesh();
    for coin in &world.coins {
        if ball_mesh.bbox().touches(&coin.mesh().bbox()) {
            events.fire(Event::Game(GEvent::CoinCollected(coin.id)));
        }
    }

    for door in &world.doors {
        if ball_mesh.bbox().touches(&door.mesh().bbox()) {
            events.fire(Event::Game(GEvent::ReachedDoor));
        }
    }

    world.time -= dt.as_secs_f64();

    if world.time < 0.0 {
        events.fire(Event::Game(GEvent::TimeLimitExpired))
    }
    
    Ok(())
}

fn time_units(time: f64) -> String {
    format!("{:01}", (time * 1.0) as u32)
}
