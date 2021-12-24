use crate::shapes::convex_mesh::Meshed;
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

pub fn render_map_normals<'a, Render, Tile, Layer>(
    map: &Map<Meshed<Tile>>, 
    layer: &Layer, 
    renderer: &mut Render, 
    up: &'a Sprite<'a>,
    down: &'a Sprite<'a>,
    left: &'a Sprite<'a>,
    right: &'a Sprite<'a>,

)
where Render: Renderer<'a, Layer>, Tile: Clone
{
    for (pos, t) in map {
        if t.mesh.normals.contains(&(-1.0, 0.0)) {
            renderer.draw(layer, left, pos.min_x, pos.min_y)
        }
        if t.mesh.normals.contains(&(1.0, 0.0)) {
            renderer.draw(layer, right, pos.min_x, pos.min_y)
        }
        if t.mesh.normals.contains(&(0.0, 1.0)) {
            renderer.draw(layer, up, pos.min_x, pos.min_y)
        }
        if t.mesh.normals.contains(&(0.0, -1.0)) {
            renderer.draw(layer, down, pos.min_x, pos.min_y)
        }
    }
}