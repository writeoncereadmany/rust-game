use crate::map::Map;

pub fn border<A>(map: &mut Map<A>, tile: A, width: usize, height: usize) 
where A: Clone {
    map.row(0, 0, width, tile.clone())
       .row(0, height - 1, width, tile.clone())
       .column(0, 0, height, tile.clone())
       .column(width - 1, 0, height, tile.clone())
       ;
}

pub fn stage1<A>(map: &mut Map<A>, tile: A) where A: Clone {
    map.row(4, 4, 4, tile.clone())
    .row(24, 4, 4, tile.clone())
    .row(1, 8, 5, tile.clone())
    .row(10, 6, 12, tile.clone())
    .row(4, 12, 6, tile.clone())
    .row(26, 8, 5, tile.clone())
    .row(22, 12, 6, tile.clone())
    .column(10, 6, 7, tile.clone())
    .column(21, 6, 7, tile.clone())
    .column(15, 10, 8, tile.clone())
    .column(16, 10, 8, tile.clone())
    ;
}