use crate::entities::ball::Ball;
use crate::map::Map;
use crate::graphics::sprite::Sprite;
use crate::fps_counter::FpsCounter;
use crate::shapes::convex_mesh::Meshed;
use crate::controller::Controller;

#[derive(Clone)]
pub enum Tile {
    STONE
}

pub struct World<'a> {
    pub ball: Ball<'a>,
    pub numbers: Vec<Sprite<'a>>,
    pub controller: Controller,
    pub map: Map<Meshed<Tile>>,
    pub fps_counter: FpsCounter
}