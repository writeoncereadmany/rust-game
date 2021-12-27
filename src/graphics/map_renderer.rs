use crate::map::Map;
use super::sprite::{ Sprited };
use super::renderer::Renderer;

pub fn render_map<'a, Render, Tile, Layer>(map: &Map<Tile>, layer: &Layer, renderer: &mut Render) 
where Render : Renderer<'a, Layer>,
      Tile : Clone + Sprited<'a>
{
    for (pos, t) in map {
        renderer.draw(layer, t.sprite(), pos.min_x, pos.min_y)
    }
}