use std::fmt::Debug;

use crate::map::Map;
use super::lo_res_renderer::LoResRenderer;

pub trait Tiled {
    fn tile(&self) -> (i32, i32);
}

pub fn render_map<'a, Tile, Layer>(map: &Map<Tile>, layer: &Layer, renderer: &mut LoResRenderer<'a, Layer>) 
where Tile : Clone + Tiled,
      Layer : Ord + Debug
{
    for (pos, t) in map {
        renderer.draw_tile(layer, t.tile(), pos.min_x, pos.min_y)
    }
}