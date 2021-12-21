use crate::entities::ball::Ball;
use crate::map::Map;
use crate::graphics::sprite::Sprite;
use crate::fps_counter::FpsCounter;
use crate::shapes::convex_mesh::ConvexMesh;
use crate::controller::Controller;

#[derive(Clone)]
pub enum Tile {
    STONE
}

#[derive(Clone)]
pub struct ColTile {
    pub tile: Tile,
    pub mesh: ConvexMesh
}

pub struct World<'a> {
    pub ball: Ball<'a>,
    pub numbers: Vec<Sprite<'a>>,
    pub controller: Controller,
    pub map: Map<ColTile>,
    pub fps_counter: FpsCounter
}