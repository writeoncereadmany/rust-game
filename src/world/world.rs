use std::time::Duration;

use image::Rgb;

use sdl2::event::Event;

use crate::app::assets::Assets;
use crate::shapes::push::Push;
use crate::entities::coin::Coin;
use crate::entities::hero::Hero;
use crate::entities::door::Door;
use crate::map::Map;
use crate::shapes::convex_mesh::Meshed;
use crate::game_loop::GameLoop;
use crate::graphics::lo_res_renderer::{ Layer, LoResRenderer };
use crate::graphics::text_renderer::SpriteFont;

#[derive(Clone)]
pub enum Tile {
    STONE
}

pub struct World<'a> {
    pub hero: Hero<'a>,
    pub coins: Vec<Coin<'a>>,
    pub doors: Vec<Door<'a>>,
    pub map: Map<Meshed<Tile>>,
    pub spritefont: SpriteFont<'a>,
    pub time: f64,
}

impl <'a> World<'a> {

    pub fn new(assets: &'a Assets<'a>, level: usize) -> Self {
        let image = &assets.level[level];
        let width = image.width();
        let height = image.height();
        let mut map = Map::new(width as usize, height as usize, 12, 12);
        let mut coins: Vec<Coin> = Vec::new();
        let mut hero: Option<Hero> = None;
        let mut doors: Vec<Door> = Vec::new();

        for x in 0..image.width() {
            for y in 0..height {
                let pixel: &Rgb<u8> = image.get_pixel(x, height - 1 - y);
                match pixel {
                    Rgb([255, 255, 255]) => { map.put(x as i32, y as i32, Tile::STONE); },
                    Rgb([255, 255, 0]) => { coins.push(Coin::new(x as f64 * 12.0, y as f64 * 12.0, 12, 12, assets))},
                    Rgb([255, 0, 0]) => { doors.push(Door::new(x as f64 * 12.0, y as f64 * 12.0, 12, 12, assets))},
                    Rgb([0, 255, 0]) => { match hero {
                        None => { hero = Some(Hero::new(x as f64 * 12.0, y as f64 * 12.0, 12, 12, assets)); }
                        Some(_) => { panic!("Multiple hero start positions defined"); }
                    }},
                    _ => { }
                }
                
            }
        }

        let map = map.add_edges();

        let spritefont =  assets.spritefont();

        World {
            hero: hero.unwrap(),
            map,
            coins,
            doors,
            spritefont,
            time: 10.0
        }
    }
}

impl <'a> GameLoop<'a, LoResRenderer<'a, Layer>> for World<'a> {
    
    fn update(&mut self, dt: Duration) -> Result<(), String> {
        
        self.hero.update(dt)?;

        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        for (_pos, t) in self.map.overlapping(&self.hero.mesh().bbox()) {
            let push = t.mesh.push(&self.hero.mesh());
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 && self.hero.dx != 0.0 && x.signum() == -self.hero.dx.signum() {
                        self.hero.x += x;
                        tot_x_push += x;
                        self.hero.dx = 0.0;
                    }
                    if y != 0.0 && self.hero.dy != 0.0 && y.signum() == -self.hero.dy.signum() {
                        self.hero.y += y;
                        tot_y_push += y;
                        self.hero.dy = 0.0;
                    }
                }
            }
        }
        self.hero.last_push = (tot_x_push, tot_y_push);

        let ball_mesh = self.hero.mesh();
        self.coins.retain(|coin| !ball_mesh.bbox().touches(&coin.mesh().bbox()));

        self.time -= dt.as_secs_f64();
        
        Ok(())
    }

    fn render(&self, renderer: &mut LoResRenderer<'a, Layer>) -> Result <(), String> {
        for coin in &self.coins {
            coin.render(renderer)?;
        }

        for door in &self.doors {
            door.render(renderer)?;
        }

        self.hero.render(renderer)?;

        self.spritefont.render(time_units(self.time), 12*15 + 4, 12 * 17 + 2, renderer, &Layer::FOREGROUND);

        Ok(())
    }

    fn on_event(&mut self, event: &Event) -> Result<(), String> {
        self.hero.on_event(event)
    }
}

fn time_units(time: f64) -> String {
    format!("{:02}", (time * 10.0) as u32)
}
