use crate::map::Map;
use super::sprite::Sprite;
use super::renderer::Renderer;

pub fn render_map<'a, Render, Tile, Layer, SpriteFunc>(map: &Map<Tile>, layer: &Layer, renderer: &mut Render, f: SpriteFunc) 
where Render : Renderer<'a, Layer>,
      Tile : Clone,
      SpriteFunc : Fn(Tile) -> &'a Sprite<'a>
{
    for (pos, t) in map {
        renderer.draw(layer, f(t), pos.min_x, pos.min_y)
    }
}