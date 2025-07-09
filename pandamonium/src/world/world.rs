use std::collections::HashSet;
use std::time::Duration;

use image::Rgb;
use image::RgbImage;

use entity::Entities;
use entity::Id;

use crate::app::assets::Assets;
use crate::app::events::*;
use crate::entities::bell::*;
use crate::entities::bubble::*;
use crate::entities::chest::*;
use crate::entities::coin::*;
use crate::entities::components::*;
use crate::entities::entity_events;
use crate::entities::flagpole::*;
use crate::entities::flashlamp::{spawn_flashlamp, TurnFlashbulbsRed, TurnFlashbulbsYellow};
use crate::entities::fruit::{spawn_fruit, Fruit};
use crate::entities::hero::*;
use crate::entities::key::*;
use crate::entities::lockbox::*;
use crate::entities::pickup::*;
use crate::entities::radial::*;
use crate::entities::spring::spawn_spring;
use crate::music::countdown::countdown;
use engine::events::*;
use engine::game_loop::*;
use engine::graphics::renderer::Renderer;
use engine::graphics::sprite::Sprite;
use engine::map::Map;
use engine::shapes::shape::collision::Collision;
use engine::shapes::shape::shape::{Shape, BLOCK};
use engine::shapes::vec2d::{Vec2d, UNIT_X, UNIT_Y};
use TileType::{DECORATION, STONE, LEDGE, WATER};
use crate::controllers::physics::translate;
use crate::entities::bubble::{spawn_bubble, BubbleHit};
use crate::entities::crumbler::spawn_crumbler;

#[derive(Clone, Eq, PartialEq)]
pub enum TileType {
    STONE,
    LEDGE,
    WATER,
    DECORATION,
}

#[derive(Clone)]
pub struct Tile {
    sprite: Sprite,
    shape: Shape,
    tile: TileType,
}

pub struct World {
    pub maps: Vec<Map<Tile>>,
    pub entities: Entities,
}

impl World {
    pub fn new(assets: &Assets, level: &String, panda_type: PandaType, events: &mut Events) -> Self {
        events.clear_schedule("world");
        let mut maps: Vec<Map<Tile>> = Vec::new();
        let mut entities = Entities::new();

        let level = assets.levels.get(level).unwrap();
        let next_level = level.next_level.clone();
        let next_bonus = level.next_bonus.clone();
        for layer in &level.layers {
            let mut map = Map::new(28, 18);
            for ((x, y), tile_ref) in layer.iter() {
                if let Some(tile) = assets.tiles.get(&tile_ref) {
                    if let Some(tile_type) = &tile.user_type {
                        match tile_type.as_str() {
                            "Wall" => {
                                map.put(*x as i32, *y as i32, Tile {
                                    sprite: Sprite::new(tile.x as i32, tile.y as i32, -1.0, &tile_ref.sheet),
                                    shape: BLOCK.translate(&(*x as f64, *y as f64)),
                                    tile: STONE,
                                });
                            }
                            "Ledge" => {
                                map.put(*x as i32, *y as i32, Tile {
                                    sprite: Sprite::new(tile.x as i32, tile.y as i32, -1.0, &tile_ref.sheet),
                                    shape: BLOCK.translate(&(*x as f64, *y as f64)),
                                    tile: LEDGE,
                                });
                            }
                            "Water" => {
                                map.put(*x as i32, *y as i32, Tile {
                                    sprite: Sprite::new(tile.x as i32, tile.y as i32, -1.0, &tile_ref.sheet),
                                    shape: BLOCK.translate(&(*x as f64, *y as f64)),
                                    tile: WATER,
                                });
                            }
                            "Hero" => {
                                let radial_balls = match panda_type {
                                    PandaType::GiantPanda => vec![4,5],
                                    PandaType::RedPanda => vec![6, 5]
                                };
                                spawn_radials(*x as f64, *y as f64, radial_balls, 6, &mut entities, events);
                                events.schedule("world", Duration::from_millis(2400), SpawnHero(*x as f64, *y as f64, panda_type));
                            }
                            "Coin" => spawn_coin(*x as f64, *y as f64, &mut entities),
                            "Lockbox" => spawn_lockbox(*x as f64, *y as f64, &mut entities),
                            "Bell" => spawn_bell(*x as f64, *y as f64, &mut entities),
                            "Chest" => spawn_chest(*x as f64, *y as f64, &mut entities),
                            "Key" => spawn_key(*x as f64, *y as f64, &mut entities),
                            "Spring" => spawn_spring(*x as f64, *y as f64, &mut entities),
                            "Flag" => spawn_flagpole(*x as f64, *y as f64, false, next_level.clone().unwrap_or("none".to_string()), &mut entities),
                            "BonusFlag" => spawn_flagpole(*x as f64, *y as f64, true, next_bonus.clone().unwrap_or("none".to_string()), &mut entities),
                            "Ruby" => spawn_ruby(*x as f64, *y as f64, &mut entities),
                            "Apple" => spawn_fruit(*x as f64, *y as f64, &Fruit::APPLE, &mut entities),
                            "Banana" => spawn_fruit(*x as f64, *y as f64, &Fruit::BANANA, &mut entities),
                            "Cherry" => spawn_fruit(*x as f64, *y as f64, &Fruit::CHERRY, &mut entities),
                            "Grapes" => spawn_fruit(*x as f64, *y as f64, &Fruit::GRAPES, &mut entities),
                            "Watermelon" => spawn_fruit(*x as f64, *y as f64, &Fruit::WATERMELON, &mut entities),
                            "Bubble" => spawn_bubble(*x as f64, *y as f64, &mut entities),
                            "Crumbler" => spawn_crumbler(*x as f64, *y as f64, &mut entities),


                            _otherwise => {}
                        }
                    } else {
                        map.put(*x as i32, *y as i32, Tile {
                            sprite: Sprite::new(tile.x as i32, tile.y as i32, -1.0, &tile_ref.sheet),
                            shape: BLOCK.translate(&(*x as f64, *y as f64)),
                            tile: DECORATION,
                        });
                    }
                }
            }
            maps.push(map);
        }

        let mut flashlamps : Vec<(i32, i32)> = Vec::new();
        for x in 17..30 { flashlamps.push((x, 19))};
        for y in 1..19 { flashlamps.push((29, 19 - y))};
        for x in 0..30 { flashlamps.push((29 - x, 0))};
        for y in 1..19 { flashlamps.push((0, y))};
        for x in 0..12 { flashlamps.push((x, 19))};

        for (i, (x, y)) in flashlamps.iter().enumerate()
        {
            let fraction_of_fulltime = i as f64 / flashlamps.len() as f64;
            let flashbulb_fire = 2.4 + (10.0 * fraction_of_fulltime);
            spawn_flashlamp((x - 1) as f64, (y - 1) as f64, flashbulb_fire, &mut entities, events);
        }

        for (x, y) in pixels(&assets.countdown, &Rgb([255, 0, 0])) { events.schedule("world", Duration::from_millis(600), SpawnBulb(x as f64, y as f64)); }
        for (x, y) in pixels(&assets.countdown, &Rgb([255, 255, 0])) { events.schedule("world", Duration::from_millis(1200), SpawnBulb(x as f64, y as f64)) }
        for (x, y) in pixels(&assets.countdown, &Rgb([0, 255, 0])) { events.schedule("world", Duration::from_millis(1800), SpawnBulb(x as f64, y as f64)) }

        for (x, y) in pixels(&assets.go, &Rgb([255, 255, 255])) { events.schedule("world", Duration::from_millis(2400), SpawnFlashBulb(x as f64, y as f64)) }

        events.schedule("world", Duration::from_millis(7400), TurnFlashbulbsYellow);
        events.schedule("world", Duration::from_millis(10400), TurnFlashbulbsRed);
        events.schedule("world", Duration::from_millis(12400), Fail);

        events.fire(ClearAudio());

        countdown(events);

        World {
            maps,
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

impl<'a> GameLoop<'a, Renderer<'a>> for World {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        for map in &self.maps {
            map.tiles().for_each(|(position, tile)|
                renderer.draw_sprite(&tile.sprite, (position.x + 1) as f64, (position.y + 1) as f64)
            );
        }

        self.entities.for_each(|(Position(x, y), sprite)| {
            renderer.draw_sprite(&sprite, x + 1.0, y + 1.0);
        });

        self.entities.for_each(|(Position(x, y), text)| {
            renderer.draw_text(&text, x, y)
        });

        Ok(())
    }

    fn event(&mut self, event: &Event, events: &mut Events) -> Result<(), String> {
        entity_events(event, &mut self.entities, events);
        event.apply(|dt| update(self, dt, events));
        Ok(())
    }
}


fn update<'a>(world: &mut World, dt: &Duration, events: &mut Events) {
    events.elapse("world", dt);
    phase(&mut world.entities, dt);
    animation_cycle(&mut world.entities);
    age(dt, &mut world.entities);
    flicker(&mut world.entities);
    map_collisions(&mut world.entities, &world.maps, events);
    item_collisions(&world.entities, events);
    apply_translations(&mut world.entities);
}

fn map_collisions(entities: &mut Entities, maps: &Vec<Map<Tile>>, events: &mut Events) {
    let obstacles: Vec<(u64, Shape)> = entities.collect().iter().map(|(Obstacle, Id(id), TranslatedMesh(shape))| (*id, shape.clone())).collect();
    entities.apply(|(Collidable, Id(movable_id), TranslatedMesh(shape), Translation(tx, ty))| {
        let (mut new_tx, mut new_ty) = (tx, ty);
        let (mut tot_px, mut tot_py) = (0.0, 0.0);
        while let Some((scenery_id, Collision { push: push@(px, py), .. })) = next_collision(maps, &obstacles, &shape, &(new_tx, new_ty)) {
            (new_tx, new_ty) = (new_tx + px, new_ty + py);
            (tot_px, tot_py) = (tot_px + px, tot_py + py);
            events.fire(SceneryCollision { movable_id, scenery_id, push });
        }
        (Translation(new_tx, new_ty), LastPush(tot_px, tot_py))
    });

    entities.apply(|(Collidable, Id(movable_id), TranslatedContextMesh(shape), Translation(tx, ty)) | {
        let mut in_water = false;
        'outer: for map in maps {
            for (_, tile) in map.overlapping(&shape, &(tx, ty)) {
                if tile.tile == WATER && shape.intersects(&tile.shape) {
                    in_water = true;
                    break 'outer;
                }
            }
        }
        events.fire(InWater(movable_id, in_water));
    });
}

fn apply_translations(entities: &mut Entities) {
    entities.apply(|(Position(x, y), LastPush(tx, ty))| {
        Position(x + tx, y + ty)
    });
    entities.apply(|(Velocity(dx, dy), LastPush(px, py))|
        Velocity(
            if px.abs() > 0.1 { 0.0 } else { dx },
            if py.abs() > 0.1 { 0.0 } else { dy },
        )
    );
    entities.apply(|(Position(x, y), ReferenceMesh(mesh))| TranslatedMesh(mesh.translate(&(x, y))));
    entities.apply(|(Position(x, y), ReferenceContextMesh(mesh))| TranslatedContextMesh(mesh.translate(&(x, y))));
}

fn next_collision(maps: &Vec<Map<Tile>>, obstacles: &Vec<(u64, Shape)>, moving: &Shape, dv: &(f64, f64)) -> Option<(u64, Collision)> {
    let mut map_collisions: Vec<(u64, Collision)> = maps.iter().map(
        |map| map.overlapping(moving, dv)
            .map(|(_, tile)| tile)
            .map(|tile| {
                if tile.tile == DECORATION || tile.tile == WATER {
                    return None;
                }
                let maybe_collision = moving.collides(&tile.shape, dv);
                if let Some(Collision { push, .. }) = maybe_collision {
                    if tile.tile == LEDGE && (push.dot(&UNIT_X).abs() > 1e-6 || push.dot(&UNIT_Y) < 0.0)
                    {
                        return None;
                    }
                }
                maybe_collision.map(|col| (0, col))
            })
            .flatten())
            .flatten()
            .collect();

    obstacles.iter()
        .map(|(id, obstacle)| moving.collides(obstacle, dv).map(|col| (*id, col)))
        .flatten()
        .for_each(|collision| map_collisions.push(collision));

    // reverse sort so earliest collisions are at the end, so we can pop
    map_collisions.sort_unstable_by(|(_, c1), (_, c2)| c1.dt.total_cmp(&c2.dt).reverse());
    map_collisions.pop()
}

fn item_collisions(entities: & Entities, events: &mut Events) {
    entities.for_each_pair(|(Hero, TranslatedMesh(hero_mesh), Translation(tx, ty)), (Pickup, Id(id), TranslatedMesh(mesh))| {
        if hero_mesh.intersects_moving(&mesh, &(*tx, *ty)) {
            events.fire(PickupCollected(*id));
        }
    });

    entities.for_each_pair(|(Hero, TranslatedMesh(hero_mesh), Translation(tx, ty)), (Bubble, Id(id), TranslatedMesh(bubble_mesh))| {
        if let Some(collision) = hero_mesh.collides(bubble_mesh, &(*tx, *ty)) {
            events.fire(BubbleHit(collision.push, *id))
        }
    });

    entities.for_each_pair(|(Hero, Id(hero_id), TranslatedMesh(hero_mesh), Translation(tx, ty)), (interaction_type, Id(other_id), TranslatedMesh(other_mesh))| {
        if hero_mesh.intersects_moving(&other_mesh, &(*tx, *ty)) {
            events.fire(Interaction {
                hero_id: *hero_id,
                other_id: *other_id,
                interaction_type: *interaction_type,
            });
        }
    });
}