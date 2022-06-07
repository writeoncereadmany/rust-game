use std::time::Duration;

use image::{ Rgb, RgbImage };

use entity::{ Id, Entities };

use crate::app::events::*;
use crate::shapes::push::Push;
use crate::entities::door::*;
use crate::entities::coin::*;
use crate::entities::timer::*;
use crate::entities::hero::*;
use crate::entities::components::*;
use crate::entities::particle::spawn_particle;
use crate::map::Map;
use crate::shapes::convex_mesh::Meshed;
use crate::events::*;
use crate::game_loop::*;
use crate::graphics::renderer::{ Renderer, Tiled };
use crate::graphics::sprite::Sprite;

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
    pub map: Map<Meshed<Tile>>,
    pub entities: Entities,
}

impl World {

    pub fn new(image: &RgbImage, panda_type: PandaType, _events: &mut Events) -> Self {
        let width = image.width();
        let height = image.height();
        let mut map : Map<Tile> = Map::new(width as usize, height as usize);
        let mut entities = Entities::new();

        for x in 0..image.width() {
            for y in 0..height {
                match pixel(image, x, y) {
                    Rgb([255, 255, 255]) => { map.put(x as i32, y as i32, Tile::STONE(tile_from_neighbours(image, x, y))); },
                    Rgb([255, 255, 0]) => { 
                        spawn_coin(x as f64, y as f64, &mut entities);
                    },
                    Rgb([255, 0, 0]) => {
                        spawn_door(x as f64, y as f64, &mut entities);
                    },
                    Rgb([0, 255, 0]) => {
                        spawn_hero(x as f64, y as f64, panda_type, &mut entities);
                    },
                    _ => { }
                }
                
            }
        }
        spawn_timer(16.0, 17.5, &mut entities);

        let map = map.add_edges();

        World {
            map,
            entities,
        }
    }
}

fn pixel(image: &RgbImage, x: u32, y: u32) -> &Rgb<u8> {
    image.get_pixel(x, image.height() - 1 - y)
}

fn tile_from_neighbours(image: &RgbImage, x: u32, y: u32) -> (i32, i32) {
    let left = x > 0 && pixel(image, x - 1, y) == &Rgb([255, 255, 255]);
    let right = x < (image.width() - 1) && pixel(image, x + 1, y) == &Rgb([255, 255, 255]);
    let bottom = y > 0 && pixel(image, x, y - 1) == &Rgb([255, 255, 255]);
    let top = y < (image.height() - 1) && pixel(image, x, y + 1) == &Rgb([255, 255, 255]);

    match (left, right, bottom, top) {
        (false, false, false, false) => (7, 3),

        (true, false, false, false) => (6, 3),
        (true, true, false, false) => (5, 3),
        (false, true, false, false) => (4, 3),

        (false, false, true, false) => (7, 0),
        (false, false, true, true) => (7, 1),
        (false, false, false, true) => (7, 2),

        (false, true, true, false) => (4, 0),
        (true, true, true, false) => (5, 0),
        (true, false, true, false) => (6, 0),
        (false, true, true, true) => (4, 1),
        (true, true, true, true) => (5, 1),
        (true, false, true, true) => (6, 1),
        (false, true, false, true) => (4, 2),
        (true, true, false, true) => (5, 2),
        (true, false, false, true) => (6, 2)
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for World {
    
    fn render(&self, renderer: &mut Renderer<'a>) -> Result <(), String> {
        renderer.draw_map(&self.map);

        renderer.draw_sprite(&Sprite::multi(2, 0, 2, 1), 15.0, 17.0);

        self.entities.for_each(|e| {
            if let (Some(Position(x, y)), Some(sprite)) = (e.get(), e.get())
            {
                renderer.draw_sprite(sprite, *x, *y);
            }
        });

        self.entities.for_each(|e| {
            if let (Some(Position(x, y)), Some(text)) = (e.get(), e.get())
            {
                renderer.draw_text(text, *x, *y)
            }
        });

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        hero_events(&mut self.entities, event, events);
        event.apply(|dt| update_timer(&mut self.entities, dt, events));
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
    age(&mut world.entities, dt);
    phase(&mut world.entities, dt);
    animation_cycle(&mut world.entities);
    map_collisions(&mut world.entities, &world.map);
    item_collisions(&world.entities, events);
}

fn map_collisions(entities: &mut Entities, map: &Map<Meshed<Tile>>) {
    entities.apply_3(|Hero, Mesh(original_mesh), &Velocity(dx, dy)| {
        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        let mut updated_mesh = original_mesh.clone();
        for (_pos, t) in map.overlapping(&updated_mesh.bbox()) {
            let push = t.mesh.push(&updated_mesh);
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 && dx != 0.0 && x.signum() == -dx.signum() {
                        updated_mesh = updated_mesh.translate(x, 0.0);
                        tot_x_push += x;
                    }
                    if y != 0.0 && dy != 0.0 && y.signum() == -dy.signum() {
                        updated_mesh = updated_mesh.translate(0.0, y);
                        tot_y_push += y;
                    }
                }
            }
        }
        LastPush(tot_x_push, tot_y_push)
    });

    entities.apply_3(|Hero, &Position(x, y), &LastPush(px, py)| {
        Position(x + px, y + py)
    });
    entities.apply_3(|Hero, &Velocity(dx, dy), &LastPush(px, py)| 
        Velocity(
            if px != 0.0 { 0.0 } else { dx }, 
            if py != 0.0 { 0.0 } else { dy },
        )
    );
    entities.apply_3(|Hero, &Position(x, y), ReferenceMesh(mesh)| Mesh(mesh.translate(x, y)));
}

fn item_collisions(entities: &Entities, events: &mut Events) {
    for (Hero, Mesh(hero_mesh)) in entities.collect_2() {

        for (Coin, &Id(id), &Position(x, y), Mesh(mesh)) in entities.collect_4() {
            if hero_mesh.bbox().touches(&mesh.bbox()) {
                events.fire(CoinCollected{ id, x, y });
            }        
        }

        for (Door, Mesh(mesh)) in entities.collect_2() {
            if hero_mesh.bbox().touches(&mesh.bbox()) {
                events.fire(ReachedDoor);
            }
        }
    }
}