use std::ops::Index;

pub struct Map<Tile>
where Tile: Clone 
{
    grid: Vec<Vec<Option<Tile>>>,
    columns: usize,
    rows: usize,
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
    type Item = (usize, usize, Tile);
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
    type Item = (usize, usize, Tile);

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
                Some(present) => return Some((x, y, present.clone())),
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