use sdl2::GameControllerSubsystem;
use sdl2::controller::GameController;
use crate::graphics::text_renderer::SpriteFont;
use crate::controller::Controller;
use crate::world::world::World;
use crate::fps_counter::FpsCounter;

pub struct App<'a> {
    pub game_controller_subsystem: GameControllerSubsystem,
    pub active_controller: Option<GameController>,
    pub world: World<'a>,
    pub fps_counter: FpsCounter,
    pub controller: Controller,
    pub spritefont: SpriteFont<'a>,
}