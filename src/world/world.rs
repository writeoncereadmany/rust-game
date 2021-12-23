use crate::entities::ball::Ball;
use crate::map::Map;
use crate::shapes::convex_mesh::Meshed;

#[derive(Clone)]
pub enum Tile {
    STONE
}

pub struct World<'a> {
    pub ball: Ball<'a>,
    pub map: Map<Meshed<Tile>>,
}