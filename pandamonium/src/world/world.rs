use std::collections::HashSet;
use std::time::Duration;

use image::Rgb;
use image::RgbImage;

use entity::Entities;
use entity::Id;

use crate::app::assets::Assets;
use crate::app::events::*;
use crate::entities::bell::*;
use crate::entities::chest::*;
use crate::entities::coin::*;
use crate::entities::components::*;
use crate::entities::entity_events;
use crate::entities::flagpole::*;
use crate::entities::hero::*;
use crate::entities::key::*;
use crate::entities::lockbox::*;
use crate::entities::particle::*;
use crate::entities::pickup::*;
use crate::entities::spring::spawn_spring;
use crate::entities::timer::*;
use crate::music::countdown::countdown;
use crate::world::world::TileType::LEDGE;
use engine::events::*;
use engine::game_loop::*;
use engine::graphics::renderer::Renderer;
use engine::graphics::sprite::Sprite;
use engine::map::Map;
use engine::shapes::shape::collision::Collision;
use engine::shapes::shape::shape::{Shape, BLOCK};
use engine::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};
use TileType::STONE;

#[derive(Clone, Eq, PartialEq)]
pub enum TileType {
    STONE, LEDGE
}

#[derive(Clone)]
pub struct Tile {
    sprite: Sprite,
    shape: Shape,
    tile: TileType
}

pub struct World {
    pub map: Map<Tile>,
    pub entities: Entities,
}

impl World {

    pub fn new(assets: &Assets, level: usize, panda_type: PandaType, events: &mut Events) -> Self {
        let image = &assets.levels[level];
        let width = image.width();
        let height = image.height();
        let mut map : Map<Tile> = Map::new(width as usize, height as usize);
        let mut entities = Entities::new();

        let tiles = pixels(image, &Rgb([255, 255, 255]));
        
        for &(x, y) in &tiles {
            let neighbours = neighbours(&tiles, x, y);
            let item = Tile {
                sprite: tile_from_neighbours(&neighbours),
                shape: BLOCK.translate(&(x as f64, y as f64)),
                tile: STONE
            };
            map.put(x, y, item);
        }

        let ledges = pixels(image, &Rgb([128, 128, 128]));
        for &(x, y) in &ledges {
            let neighbours = neighbours(&ledges, x, y);
            let item = Tile {
                sprite: ledge_from_neighbours(&neighbours),
                shape: BLOCK.translate(&(x as f64, y as f64)),
                tile: LEDGE
            };
            map.put(x, y, item);
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
 
fn tile_from_neighbours(neighbours: &Neighbours) -> Sprite {
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

    Sprite::new(tx, ty, -1.0, "Sprites")
}

fn ledge_from_neighbours(neighbours: &Neighbours) -> Sprite {
    let tx = match (neighbours.left, neighbours.right) {
        (false, true) => 4,
        (true, true) => 5,
        (true, false) => 6,
        (false, false) => 7 
    };

    Sprite::new(tx, 4, -1.0, "Sprites")
}

impl <'a> GameLoop<'a, Renderer<'a>> for World {
    
    fn render(&self, renderer: &mut Renderer<'a>) -> Result <(), String> {
        self.map.tiles().for_each(|(position, tile)|
            renderer.draw_sprite(&tile.sprite, position.x as f64, position.y as f64)
        );

        renderer.draw_sprite(&Sprite::multi(2, 0, 0.0, 2, 1, "Sprites"), 14.0, 19.0);

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

fn map_collisions(entities: &mut Entities, map: &Map<Tile>) {
    let obstacles: Vec<Shape> = entities.collect().iter().map(|(Obstacle, TranslatedMesh(shape))| shape.clone()).collect();
    entities.apply(|(Collidable, TranslatedMesh(shape), Translation(tx, ty))| {
        let (mut new_tx, mut new_ty) = (tx, ty);
        let (mut tot_px, mut tot_py) = (0.0, 0.0);
        while let Some(Collision { push: (px, py), .. }) = next_collision(map, &obstacles, &shape, &(new_tx, new_ty)) {
            (new_tx, new_ty) = (new_tx + px, new_ty + py);
            (tot_px, tot_py) = (tot_px + px, tot_py + py);
        }
        (Translation(new_tx, new_ty), LastPush(tot_px, tot_py))
    });

    entities.apply(|(Position(x, y), LastPush(px, py))| {
        Position(x + px, y + py)
    });
    entities.apply(|(Velocity(dx, dy), LastPush(px, py))| 
        Velocity(
            if px.abs() > 1e-6 { 0.0 } else { dx },
            if py.abs() > 1e-6 { 0.0 } else { dy },
        )
    );
    entities.apply(|(Position(x, y), ReferenceMesh(mesh))| TranslatedMesh(mesh.translate(&(x, y))));
}

fn next_collision(map: &Map<Tile>, obstacles: &Vec<Shape>, moving: &Shape, dv: &(f64, f64)) -> Option<Collision> {
    let mut map_collisions: Vec<Collision> = map.overlapping(moving, dv)
        .map(|(_, tile)| tile)
        .map(|tile| {
            let maybe_collision = moving.collides(&tile.shape, dv);
            if let Some(Collision { push, ..}) = maybe_collision {
                if tile.tile == LEDGE && (push.dot(&UNIT_X).abs() > 1e-6 || push.dot(&UNIT_Y) < 0.0)
                {
                    return None;
                }
            }
            maybe_collision
        })
        .flatten()
        .collect();

    obstacles.iter()
        .map(|obstacle| moving.collides(obstacle, dv))
        .flatten()
        .for_each(|collision| map_collisions.push(collision));

    // reverse sort so earliest collisions are at the end, so we can pop
    map_collisions.sort_unstable_by(|c1, c2| c1.dt.total_cmp(&c2.dt).reverse());
    map_collisions.pop()
}

fn item_collisions(entities: &Entities, events: &mut Events) {
    entities.for_each_pair(|(Hero, TranslatedMesh(hero_mesh), Translation(tx, ty)), (Pickup, Id(id), TranslatedMesh(mesh))| {
        if hero_mesh.intersects_moving(&mesh, &(*tx, *ty)) {
            events.fire(PickupCollected(*id));
        }
    });

    entities.for_each_pair(|(Hero, Id(hero_id), TranslatedMesh(hero_mesh), Translation(tx, ty)), (interaction_type, Id(other_id), TranslatedMesh(other_mesh))| {
        if hero_mesh.intersects_moving(&other_mesh, &(*tx, *ty)) {
            events.fire(Interaction { 
                hero_id: *hero_id, 
                other_id: *other_id, 
                interaction_type: *interaction_type 
            });
        }
    });
}