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
use crate::shapes::convex_mesh::{ Meshed, ConvexMesh };
use crate::events::*;
use crate::game_loop::*;
use crate::graphics::renderer::{ Renderer, Tiled };
use crate::graphics::sprite::Sprite;

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

    pub fn new(image: &RgbImage, panda_type: PandaType, _events: &mut Events) -> Self {
        let width = image.width();
        let height = image.height();
        let mut map : Map<Meshed<Tile>> = Map::new(width as usize, height as usize);
        let mut entities = Entities::new();

        for x in 0..image.width() {
            for y in 0..height {
                match pixel(image, x, y) {
                    Rgb([255, 255, 255]) => { 
                        let neighbours = neighbours(image, x, y);
                        let item = Tile::STONE(tile_from_neighbours(&neighbours));
                        let mesh = mesh_from_neighbours(x as f64, y as f64, &neighbours);
                        map.put(x as i32, y as i32, Meshed{ item, mesh }); 
                    },
                    Rgb([255, 255, 0]) => { 
                        spawn_coin(x as f64, y as f64, &mut entities);
                    },
                    Rgb([255, 0, 0]) => {
                        spawn_door(x as f64, y as f64, &mut entities);
                    },
                    Rgb([0, 255, 0]) => {
                        spawn_hero(x as f64, y as f64, panda_type, &mut entities);
                    },
                    Rgb([128, 128, 128]) => {
                         map.put(x as i32, y as i32, Meshed { item : Tile::LEDGE((6, 4)), mesh: ledge_mesh(x as f64, y as f64) });
                    }
                    _ => { }
                }
                
            }
        }
        spawn_timer(16.0, 17.5, &mut entities);

        World {
            map,
            entities,
        }
    }
}

fn pixel(image: &RgbImage, x: u32, y: u32) -> &Rgb<u8> {
    image.get_pixel(x, image.height() - 1 - y)
}

struct Neighbours {
    above: bool,
    below: bool,
    left: bool,
    right: bool
}

fn neighbours(image: &RgbImage, x: u32, y: u32) -> Neighbours {
    Neighbours {
        left: x > 0 && pixel(image, x - 1, y) == &Rgb([255, 255, 255]),
        right: x < (image.width() - 1) && pixel(image, x + 1, y) == &Rgb([255, 255, 255]),
        below: y > 0 && pixel(image, x, y - 1) == &Rgb([255, 255, 255]),
        above: y < (image.height() - 1) && pixel(image, x, y + 1) == &Rgb([255, 255, 255]),
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

// When considering pushouts, we don't want to push more than an entity has moved that frame, because that can
// lead to a jump from being on one side of an edge to another. This should never happen with solid objects, but
// with one-way edges like ledges, we should only push if they started off on the outside of the ledge. However,
// float math is imprecise here, so this caters for slight math errors. This does mean we can force a jump from inside
// an edge to outside it, but only by this many units.
const TRANSLATE_EPSILON: f64 = 0.01;

fn map_collisions(entities: &mut Entities, map: &Map<Meshed<Tile>>) {
    entities.apply_4(|Hero, Mesh(original_mesh), &Velocity(dx, dy), &Translation(tx, ty)| {
        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        let mut updated_mesh = original_mesh.clone();
        for (_pos, t) in map.overlapping(&updated_mesh.bbox()) {
            let push = t.mesh.push(&updated_mesh);
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 && dx != 0.0 && x.signum() == -dx.signum() && x.abs() <= (tx.abs() + TRANSLATE_EPSILON) {
                        updated_mesh = updated_mesh.translate(x, 0.0);
                        tot_x_push += x;
                    }
                    if y != 0.0 && dy != 0.0 && y.signum() == -dy.signum() && y.abs() <= (ty.abs() + TRANSLATE_EPSILON) {
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