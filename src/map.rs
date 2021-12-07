use std::ops::Index;

pub struct Map<Tile>
where Tile: Clone 
{
    grid: Vec<Vec<Option<Tile>>>,
    tile_height: u32,
    tile_width: u32,
    columns: usize,
    rows: usize,
}

pub struct Position {
    pub grid_x: i32,
    pub grid_y: i32,
    pub min_x: i32,
    pub max_x: i32,
    pub min_y: i32,
    pub max_y: i32
}

impl <Tile> Map<Tile> 
where Tile: Clone {
    pub fn new(columns: usize, rows: usize, tile_width: u32, tile_height: u32) -> Self {
        Map {
            grid: vec![vec![Option::None; rows]; columns],
            tile_height,
            tile_width,
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

    pub fn overlapping(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> MapIter<Tile> {
        let grid_min_x = constrain(f64::floor(min_x / self.tile_width as f64), 0, self.columns - 1);
        let grid_max_x = constrain(f64::floor(max_x / self.tile_width as f64), 0, self.columns - 1);
        let grid_min_y = constrain(f64::floor(min_y / self.tile_width as f64), 0, self.rows - 1);
        let grid_max_y = constrain(f64::floor(max_y / self.tile_width as f64), 0, self.rows - 1);

        MapIter {
            map: self,
            x: grid_min_x,
            y: grid_min_y,
            min_x: grid_min_x,
            max_x: grid_max_x,
            max_y: grid_max_y
        }
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
                        grid_x: x as i32,
                        grid_y: y as i32,
                        min_x: x as i32 * self.map.tile_width as i32,
                        max_x: (x + 1) as i32 * self.map.tile_width as i32,
                        min_y: y as i32 * self.map.tile_height as i32,
                        max_y: (y + 1) as i32 * self.map.tile_height as i32
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
        let mut map : Map<u32> = Map::new(10, 10, 100, 100);
        // fill entire map with tiles
        for i in 0..10 {
            map.row(0, i, 10, 3);
        }

        let mut iterated : Vec<(i32, i32)> = Vec::new();

        for (pos, _tile) in map.overlapping(250.0, 450.0, 350.0, 650.0) {
            iterated.push((pos.grid_x, pos.grid_y));
        }

        assert_eq!(iterated, vec![
            (2, 3), (3, 3), (4, 3), 
            (2, 4), (3, 4), (4, 4), 
            (2, 5), (3, 5), (4, 5), 
            (2, 6), (3, 6), (4, 6)])
    }
}