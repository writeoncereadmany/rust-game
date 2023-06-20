use std::time::Duration;
use std::collections::HashSet;

use image::RgbImage;
use image::Rgb;

use entity::not;
use entity::Entities;
use entity::Id;

use engine::graphics::renderer::Renderer;
use engine::graphics::sprite::Sprite;
use engine::events::*;
use engine::shapes::push::Push;
use engine::shapes::convex_mesh::{ Meshed, ConvexMesh };
use engine::game_loop::*;
use engine::map::{ Map, Tiled };

use crate::app::assets::Assets;
use crate::app::events::*;
use crate::entities::entity_events;
use crate::entities::spring::spawn_spring;
use crate::music::countdown::countdown;
use crate::entities::flagpole::*;
use crate::entities::bell::*;
use crate::entities::chest::*;
use crate::entities::key::*;
use crate::entities::coin::*;
use crate::entities::timer::*;
use crate::entities::hero::*;
use crate::entities::pickup::*;
use crate::entities::lockbox::*;
use crate::entities::components::*;
use crate::entities::particle::*;

#[derive(Clone)]
pub enum Tile {
    STONE((i32, i32)),
    LEDGE((i32, i32))
}

impl Tiled for Tile {
    fn tile(&self) -> (i32, i32) {
        match self {
            Tile::STONE(tile) => *tile,
            Tile::LEDGE(tile) => *tile
        }
    }
}

pub struct World {
    pub map: Map<Meshed<Tile>>,
    pub entities: Entities,
}

impl World {

    pub fn new(assets: &Assets, level: usize, panda_type: PandaType, events: &mut Events) -> Self {
        let image = &assets.levels[level];
        let width = image.width();
        let height = image.height();
        let mut map : Map<Meshed<Tile>> = Map::new(width as usize, height as usize);
        let mut entities = Entities::new();

        let tiles = pixels(image, &Rgb([255, 255, 255]));
        
        for &(x, y) in &tiles {
            let neighbours = neighbours(&tiles, x as i32, y as i32);
            let item = Tile::STONE(tile_from_neighbours(&neighbours));
            let mesh = mesh_from_neighbours(x as f64, y as f64, &neighbours);
            map.put(x as i32, y as i32, Meshed{ item, mesh }); 
        }

        let ledges = pixels(image, &Rgb([128, 128, 128]));
        for &(x, y) in &ledges {
            let neighbours = neighbours(&ledges, x as i32, y as i32);
            let item = Tile::LEDGE(ledge_from_neighbours(&neighbours));
            let mesh = ledge_mesh(x as f64, y as f64);
            map.put(x as i32, y as i32, Meshed{ item, mesh });
        }

        for (x, y) in pixels(image, &Rgb([255, 255, 0])) { spawn_coin(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([255, 0, 0])) { spawn_flagpole(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([255,0,255])) { spawn_bell(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([0,255,255])) { spawn_key(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([255,127,0])) { spawn_chest(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([127,0,255])) { spawn_lockbox(x as f64, y as f64, &mut entities); }
        for (x, y) in pixels(image, &Rgb([255,0,127])) { spawn_spring(x as f64, y as f64, &mut entities); }


        for (x, y) in pixels(image, &Rgb([0, 255, 0])) { 
            spawn_shadow(x as f64, y as f64, panda_type, &mut entities, events);
            events.schedule(Duration::from_millis(2400), SpawnHero(x as f64, y as f64, panda_type)); 
        }
       
        for (x, y) in pixels(&assets.countdown, &Rgb([255, 0, 0])) { events.schedule(Duration::from_millis(600), SpawnBulb(x as f64, y as f64)); }
        for (x, y) in pixels(&assets.countdown, &Rgb([255, 255, 0])) { events.schedule(Duration::from_millis(1200), SpawnBulb(x as f64, y as f64))}
        for (x, y) in pixels(&assets.countdown, &Rgb([0, 255, 0])) { events.schedule(Duration::from_millis(1800), SpawnBulb(x as f64, y as f64))}
 
        for (x, y) in pixels(&assets.go, &Rgb([255, 255, 255])) { events.schedule(Duration::from_millis(2400), SpawnFlashBulb(x as f64, y as f64))}

        events.schedule(Duration::from_millis(2400), SpawnTimer(15.0, 19.5));

        events.fire(ClearAudio());

        countdown(events);

        World {
            map,
            entities,
        }
    }
}

fn pixels(image: &RgbImage, color: &Rgb<u8>) -> HashSet<(i32, i32)> {
    let mut pixels = HashSet::new();
    
    let height = image.height() as i32;

    for x in 0..image.width() {
        for y in 0..image.height() {
            if image.get_pixel(x, y) == color {
                pixels.insert((x as i32, height - 1 - y as i32));
            }
        }
    }
    pixels
}

struct Neighbours {
    above: bool,
    below: bool,
    left: bool,
    right: bool
}

fn neighbours(pixels: &HashSet<(i32, i32)>, x: i32, y: i32) -> Neighbours {
    Neighbours {
        left: pixels.contains(&(x - 1, y)),
        right: pixels.contains(&(x + 1, y)),
        below: pixels.contains(&(x, y - 1)),
        above: pixels.contains(&(x, y + 1)),
    }
}
 
fn tile_from_neighbours(neighbours: &Neighbours) -> (i32, i32) {
    let tx = match (neighbours.left, neighbours.right) {
        (false, true) => 4,
        (true, true) => 5,
        (true, false) => 6,
        (false, false) => 7 
    };

    let ty = match (neighbours.above, neighbours.below) {
        (false, true) => 0,
        (true, true) => 1,
        (true, false) => 2,
        (false, false) => 3
    };

    (tx, ty)
}

fn ledge_from_neighbours(neighbours: &Neighbours) -> (i32, i32) {
    let tx = match (neighbours.left, neighbours.right) {
        (false, true) => 4,
        (true, true) => 5,
        (true, false) => 6,
        (false, false) => 7 
    };

    (tx, 4)
}

fn mesh_from_neighbours(x: f64, y: f64, neighbours: &Neighbours) -> ConvexMesh {
    let points = vec![(x, y), (x + 1.0, y), (x + 1.0, y + 1.0), (x, y + 1.0)];
    let mut normals : Vec<(f64, f64)> = Vec::new();
    if !neighbours.left { normals.push((-1.0, 0.0));}
    if !neighbours.right { normals.push((1.0, 0.0));}
    if !neighbours.above { normals.push((0.0, 1.0));}
    if !neighbours.below { normals.push((0.0, -1.0));}

    ConvexMesh::new(points, normals)
}

fn ledge_mesh(x: f64, y: f64) -> ConvexMesh {
    let points = vec![(x, y), (x + 1.0, y), (x + 1.0, y + 1.0), (x, y + 1.0)];
    let normals = vec![(0.0, 1.0)];

    ConvexMesh::new(points, normals)
}

impl <'a> GameLoop<'a, Renderer<'a>> for World {
    
    fn render(&self, renderer: &mut Renderer<'a>) -> Result <(), String> {
        self.map.draw(renderer);

        renderer.draw_sprite(&Sprite::multi(2, 0, 0.0, 2, 1), 14.0, 19.0);

        self.entities.for_each(|(Position(x, y), sprite)| {
            renderer.draw_sprite(&sprite, x, y);
        });

        self.entities.for_each(|(Position(x, y), text)| {
            renderer.draw_text(&text, x, y)
        });

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        entity_events(event, &mut self.entities, events);
        event.apply(|dt| update_timer(&mut self.entities, dt, events));
        event.apply(|dt| update(self, dt, events));
        Ok(())
    }
}



fn update<'a>(world: &mut World, dt: &Duration, events: &mut Events) {
    phase(&mut world.entities, dt);
    animation_cycle(&mut world.entities);
    flicker(&mut world.entities);
    map_collisions(&mut world.entities, &world.map);
    item_collisions(&world.entities, events);
}

fn map_collisions(entities: &mut Entities, map: &Map<Meshed<Tile>>) {
    let collidables = entities.collect();
    entities.apply(|(Collidable, Mesh(original_mesh), Translation(tx, ty))| {
        let (mut tot_x_push, mut tot_y_push) = map.push(&original_mesh, &(tx, ty)).unwrap_or((0.0, 0.0));
        let (mut utx, mut uty) = (tx + tot_x_push, ty + tot_y_push);
        let mut updated_mesh = original_mesh.translate(tot_x_push, tot_y_push);
        for (Obstacle, Mesh(mesh)) in &collidables {
            let push = mesh.push(&updated_mesh, &(utx, uty));
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 {
                        updated_mesh = updated_mesh.translate(x, 0.0);
                        tot_x_push += x;
                        utx += x;
                    }
                    if y != 0.0 {
                        updated_mesh = updated_mesh.translate(0.0, y);
                        tot_y_push += y;
                        uty += y;
                    }
                }
            }
        }
        (LastPush(tot_x_push, tot_y_push), not::<Translation>())
    });

    entities.apply(|(Position(x, y), LastPush(px, py))| {
        Position(x + px, y + py)
    });
    entities.apply(|(Velocity(dx, dy), LastPush(px, py))| 
        Velocity(
            if px != 0.0 { 0.0 } else { dx }, 
            if py != 0.0 { 0.0 } else { dy },
        )
    );
    entities.apply(|(Position(x, y), ReferenceMesh(mesh))| Mesh(mesh.translate(x, y)));
}

fn item_collisions(entities: &Entities, events: &mut Events) {
    entities.for_each_pair(|(Hero, Mesh(hero_mesh)), (Pickup, Id(id), Mesh(mesh))| {
        if hero_mesh.bbox().touches(&mesh.bbox()) {
            events.fire(PickupCollected(*id));
        }
    });

    entities.for_each_pair(|(Hero, Id(hero_id), Mesh(hero_mesh)), (interaction_type, Id(other_id), Mesh(other_mesh))| {
        if hero_mesh.bbox().touches(&other_mesh.bbox()) {
            events.fire(Interaction { 
                hero_id: *hero_id, 
                other_id: *other_id, 
                interaction_type: *interaction_type });
        }
    });
}