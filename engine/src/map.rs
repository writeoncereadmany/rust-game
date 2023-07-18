use std::ops::Index;

use crate::graphics::renderer::Renderer;
use crate::graphics::sprite::Sprite;

use crate::shapes::convex_mesh::{Meshed, Mesh};
use crate::shapes::push::Push;
use crate::shapes::vec2d::{UNIT_X, UNIT_Y};

pub trait Tiled {
    fn tile(&self) -> (i32, i32);
}

pub struct Map<Tile>
where Tile: Clone 
{
    grid: Vec<Vec<Option<Tile>>>,
    columns: usize,
    rows: usize,
}

impl <A> Tiled for Meshed<A> where A: Clone + Tiled {
    fn tile(&self) -> (i32, i32) {
        self.item.tile()
    }
}

pub struct Position {
    pub x: i32,
    pub y: i32
}

impl <Tile> Map<Tile> 
where Tile: Clone {
    pub fn new(columns: usize, rows: usize) -> Self {
        Map {
            grid: vec![vec![Option::None; rows]; columns],
            columns,
            rows
        }
    }

    pub fn get(&self, x: i32, y: i32) -> &Option<Tile> {
        if x < 0 || x >= self.columns as i32 || y < 0 || y >= self.rows as i32 {
            &None
        } else {
            &self.grid[x as usize][y as usize]
        }
    }

    pub fn put(&mut self, x: i32, y: i32, tile: Tile) -> &mut Self {
        self.grid[x as usize][y as usize] = Option::Some(tile.clone());
        self
    }

    pub fn row(&mut self, x: usize, y: usize, len: usize, tile: Tile) -> &mut Self {
        for i in 0..len {
            self.grid[x + i][y] = Option::Some(tile.clone());
        }
        self
    }

    pub fn column(&mut self, x: usize, y: usize, len: usize, tile: Tile) -> &mut Self {
        for i in 0..len  {
            self.grid[x][y+i] = Option::Some(tile.clone());
        }
        self
    }

    pub fn overlapping(&self, bbox: &Mesh, relative_motion: &(f64, f64)) -> MapIter<Tile> {
        let (left, right) = bbox.project(&UNIT_X, relative_motion);
        let (bottom, top) = bbox.project(&UNIT_Y, relative_motion);
        let grid_min_x = constrain(f64::floor(left), 0, self.columns - 1);
        let grid_max_x = constrain(f64::floor(right), 0, self.columns - 1);
        let grid_min_y = constrain(f64::floor(bottom), 0, self.rows - 1);
        let grid_max_y = constrain(f64::floor(top), 0, self.rows - 1);

        MapIter {
            map: self,
            x: grid_min_x,
            y: grid_min_y,
            min_x: grid_min_x,
            max_x: grid_max_x,
            max_y: grid_max_y
        }
    }

    pub fn draw(&self, renderer: &mut Renderer) 
    where Tile : Clone + Tiled,
    {
        for (pos, t) in self {
            let (x, y) = t.tile();
            renderer.draw_sprite(&Sprite::new(x, y, 0.0), pos.x as f64, pos.y as f64);
        }
    }
}

impl <Tile> Push<Mesh> for Map<Meshed<Tile>> where Tile: Clone {

    fn push(&self, original_mesh: &Mesh, relative_motion: &(f64, f64)) -> Option<(f64, f64)> {
        let (mut tot_x_push, mut tot_y_push) = (0.0, 0.0);
        let (mut rel_x, mut rel_y) = relative_motion;
        let mut updated_mesh = original_mesh.clone();
        for (_pos, t) in self.overlapping(&updated_mesh, relative_motion) {
            let push = t.mesh.push(&updated_mesh, &(rel_x, rel_y));
            match push {
                None => {},
                Some((x, y)) => {
                    if x != 0.0 {
                        updated_mesh = updated_mesh.translate(x, 0.0);
                        tot_x_push += x;
                        rel_x += x;
                    }
                    if y != 0.0 {
                        updated_mesh = updated_mesh.translate(0.0, y);
                        tot_y_push += y;
                        rel_y += y;
                    }
                }
            }
        }
        if tot_x_push == 0.0 && tot_y_push == 0.0 { None } else { Some((tot_x_push, tot_y_push)) }
    }
}


fn constrain(value: f64, min: usize, max: usize) -> usize {
    if value < min as f64 {
        min 
    } else if value > max as f64 {
        max
    } else {
        value as usize
    }
}


impl <'a, Tile> IntoIterator for &'a Map<Tile> 
where Tile: Clone {
    type Item = (Position, Tile);
    type IntoIter = MapIter<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        MapIter {
            map: self,
            x: 0,
            y: 0,
            min_x: 0,
            max_x: self.columns - 1,
            max_y: self.rows - 1
        }
    }
}

pub struct MapIter<'a, Tile> 
where Tile: Clone {
    map : &'a Map<Tile>,
    x: usize,
    y: usize,
    min_x: usize,
    max_x: usize,
    max_y: usize
}

impl <'a, Tile> Iterator for MapIter<'a, Tile>
where Tile: Clone {
    type Item = (Position, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if self.y > self.max_y {
                return None;
            }
            let x = self.x;
            let y = self.y;
            let tile = self.map.grid[x][y].as_ref();
            if self.x < self.max_x {
                self.x += 1;
            } else {
                self.x = self.min_x;
                self.y += 1;
            } 
            match tile {
                Some(present) => {
                    let position = Position {
                        x: x as i32,
                        y: y as i32
                    };
                    return Some((position, present.clone()))
                },
                None => {}
            }
        }
    }
}

impl <Tile> Index<usize> for Map<Tile> 
where Tile: Clone {
    type Output = Vec<Option<Tile>>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.grid[idx]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_iterate_only_over_subset(){
        let mut map : Map<u32> = Map::new(10, 10);
        // fill entire map with tiles
        for i in 0..10 {
            map.row(0, i, 10, 3);
        }

        let mut iterated : Vec<(i32, i32)> = Vec::new();

        for (pos, _tile) in map.overlapping(&Mesh::rect(2.5, 3.5, 2.0, 3.0), &(0.0, 0.0)) {
            iterated.push((pos.x, pos.y));
        }

        assert_eq!(iterated, vec![
            (2, 3), (3, 3), (4, 3), 
            (2, 4), (3, 4), (4, 4), 
            (2, 5), (3, 5), (4, 5), 
            (2, 6), (3, 6), (4, 6)])
    }
}