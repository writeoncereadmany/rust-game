use std::time::Duration;

use component_derive::Event;
use engine::events::*;
use engine::game_loop::*;
use engine::graphics::renderer::{align, Renderer, Text};
use engine::graphics::sprite::Sprite;
use crate::app::assets::Assets;
use crate::app::events::*;
use crate::entities::hero::PandaType;
use crate::world::world::World;

pub struct Game<'a> {
    pub assets: &'a Assets<'a>,
    pub world: World,
    pub score: u32,
    pub panda_type: PandaType,
    pub pause: f64,
}
#[derive(Event)]
struct Pause(f64);

#[derive(Event)]
struct NewLevel(String);

impl<'a> Game<'a> {
    pub fn new(panda_type: PandaType, assets: &'a Assets<'a>, events: &mut Events) -> Game<'a> {
        let world: World = World::new(
            &assets,
            &"0001".to_string(),
            panda_type,
            events);

        Game {
            assets: &assets,
            world,
            score: 0,
            panda_type,
            pause: 0.0,
        }
    }
}

impl<'a> GameLoop<'a, Renderer<'a>> for Game<'a> {
    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        self.world.render(renderer)?;
        renderer.draw_sprite(&Sprite::new(8, 6, 0.0, "Walls"), 13.0, 19.0);
        renderer.draw_sprite(&Sprite::new(9, 6, 0.0, "Walls"), 14.0, 19.0);
        renderer.draw_sprite(&Sprite::new(9, 6, 0.0, "Walls"), 15.0, 19.0);
        renderer.draw_sprite(&Sprite::new(10, 6, 0.0, "Walls"), 16.0, 19.0);

        renderer.draw_text(
            &Text { text: self.score.to_string(), justification: align::RIGHT | align::MIDDLE },
            16.75,
            19.5);
        Ok(())
    }

    fn event(&mut self, event: &Event, mut events: &mut Events) -> Result<(), String> {
        event.apply(|score| {
            match score {
                Score::Points(p) => self.score += *p,
                Score::Double => self.score *= 2
            }
        });

        event.apply(|TimeLimitExpired| {
            events.fire(Pause(2.0));
            events.schedule(Duration::from_secs_f64(2.0), GameOver(self.score));
        });

        event.apply(|ReachedDoor(next_level)| {
            if self.assets.levels.contains_key(next_level) {
                events.fire(Pause(0.5));
                events.schedule(Duration::from_secs_f64(0.5), NewLevel(next_level.clone()));
            } else {
                events.fire(Pause(2.0));
                events.schedule(Duration::from_secs_f64(2.0), GameOver(self.score));
            }
        });

        event.apply(|NewLevel(level)| {
            self.world = World::new(
                &self.assets,
                level,
                self.panda_type,
                &mut events);
        });

        event.apply(|Pause(pause)| {
            self.pause = *pause;
        });

        if let Some(duration) = event.unwrap::<Duration>() {
            self.pause -= duration.as_secs_f64();
            self.pause = f64::max(self.pause, 0.0);
            if self.pause > 0.0 {
                return Ok(());
            }
        }
        self.world.event(event, events)
    }
}