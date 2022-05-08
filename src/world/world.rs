use std::time::Duration;

use image::{ Rgb, RgbImage };

use entity::{ Id, Entities };

use crate::app::events::*;
use crate::shapes::push::Push;
use crate::entities::coin::spawn_coin;
use crate::entities::hero::*;
use crate::entities::components::*;
use crate::entities::door::Door;
use crate::entities::particle::spawn_particle;
use crate::map::Map;
use crate::controller::Controller;
use crate::shapes::convex_mesh::Meshed;
use crate::events::*;
use crate::game_loop::*;
use crate::graphics::renderer::{ Renderer, align, Tiled };

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
    pub doors: Vec<Door>,
    pub map: Map<Meshed<Tile>>,
    pub entities: Entities,
    pub time: f64,
}

impl World {

    pub fn new(image: &RgbImage, controller: Controller, panda_type: PandaType, events: &mut Events) -> Self {
        let width = image.width();
        let height = image.height();
        let mut map : Map<Tile> = Map::new(width as usize, height as usize);
        let mut hero: Option<Hero> = None;
        let mut doors: Vec<Door> = Vec::new();
        let mut entities = Entities::new();

        for x in 0..image.width() {
            for y in 0..height {
                let pixel: &Rgb<u8> = image.get_pixel(x, height - 1 - y);
                match pixel {
                    Rgb([255, 255, 255]) => { map.put(x as i32, y as i32, Tile::STONE((0, 1))); },
                    Rgb([255, 255, 0]) => { 
                        spawn_coin(x as f64, y as f64, &mut entities);
                    },

                    Rgb([255, 0, 0]) => { doors.push(Door::new(x as f64, y as f64))},
                    Rgb([0, 255, 0]) => { match hero {
                        None => { hero = Some(Hero::new(
                            x as f64, 
                            y as f64, 
                            controller,
                            panda_type)); }
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
            doors,
            entities,
            time: 10.0,
        }
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for World {
    
    fn render(&self, renderer: &mut Renderer<'a>) -> Result <(), String> {
        renderer.draw_map(&self.map);

        renderer.draw_multitile((2, 0), (2, 1), 15.0, 17.0);

        for door in &self.doors {
            door.render(renderer)?;
        }

        self.hero.render(renderer)?;

        self.entities.for_each(|e| {
            if let (Some(Position(x, y)), Some(Tile(tile))) = (e.get(), e.get())
            {
                renderer.draw_tile(*tile, *x, *y);
            }
        });

        renderer.draw_text(
            time_units(self.time), 
            16.0, 
            17.5, 
            align::CENTER & align::MIDDLE);

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        self.hero.event(event, events)?;

        event.apply(|dt| update(self, dt, events));
        event.apply(|Destroy(id)| self.entities.delete(id));
        event.apply(|CoinCollected{ x, y, id }| {
            spawn_particle(*x, *y, &mut self.entities, events);
            self.entities.delete(id);
        });

        Ok(())
    }
}

fn update<'a>(world: &mut World, dt: &Duration, events: &mut Events) {
        
    world.entities.apply(|Age(age)| Age(age + dt.as_secs_f64()));
    world.entities.apply_2(|Period(period), Phase(phase)| Phase((phase + (dt.as_secs_f64() / period)) % 1.0));
    world.entities.apply_2(|Phase(phase), AnimationCycle(frames)| Tile(next_frame(phase, frames)));
    world.entities.apply_2(|Position(x, y), ReferenceMesh(mesh)| Mesh(mesh.translate(*x, *y)));

    let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
    let mut hero_mesh = world.hero.mesh().clone();
    for (_pos, t) in world.map.overlapping(&hero_mesh.bbox()) {
        let push = t.mesh.push(&hero_mesh);
        match push {
            None => {},
            Some((x, y)) => {
                if x != 0.0 && world.hero.dx != 0.0 && x.signum() == -world.hero.dx.signum() {
                    hero_mesh = hero_mesh.translate(x, 0.0);
                    tot_x_push += x;
                }
                if y != 0.0 && world.hero.dy != 0.0 && y.signum() == -world.hero.dy.signum() {
                    hero_mesh = hero_mesh.translate(0.0, y);
                    tot_y_push += y;
                }
            }
        }
    }
    world.hero.x += tot_x_push;
    world.hero.y += tot_y_push;
    if tot_x_push != 0.0 { world.hero.dx = 0.0; }
    if tot_y_push != 0.0 { world.hero.dy = 0.0; }
    world.hero.last_push = (tot_x_push, tot_y_push);

    let hero_mesh = world.hero.mesh();
    for (Id(id), Position(x, y), Mesh(mesh)) in &world.entities.collect_3() {
        if hero_mesh.bbox().touches(&mesh.bbox()) {
            events.fire(CoinCollected{ id: *id, x: *x, y: *y });
        }
    }

    for door in &world.doors {
        if hero_mesh.bbox().touches(&door.mesh().bbox()) {
            events.fire(ReachedDoor);
        }
    }

    world.time -= dt.as_secs_f64();

    if world.time < 0.0 {
        events.fire(TimeLimitExpired)
    }
}

fn time_units(time: f64) -> String {
    format!("{:01}", (time * 10.0) as u32)
}
