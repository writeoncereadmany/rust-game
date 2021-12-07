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
}

impl <'a, Tile> IntoIterator for &'a Map<Tile> 
where Tile: Clone {
    type Item = (i32, i32, Tile);
    type IntoIter = MapIter<'a, Tile>;

    fn into_iter(self) -> Self::IntoIter {
        MapIter {
            map: self,
            x: 0,
            y: 0
        }
    }
}

pub struct MapIter<'a, Tile> 
where Tile: Clone {
    map : &'a Map<Tile>,
    x: usize,
    y: usize,
}

impl <'a, Tile> Iterator for MapIter<'a, Tile>
where Tile: Clone {
    type Item = (i32, i32, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        loop
        {
            if self.y >= self.map.rows {
                return None;
            }
            let x = self.x;
            let y = self.y;
            let tile = self.map.grid[x][y].as_ref();
            if self.x < self.map.columns - 1 {
                self.x += 1;
            } else {
                self.x = 0;
                self.y += 1;
            } 
            match tile {
                Some(present) => return Some((x as i32, y as i32, present.clone())),
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