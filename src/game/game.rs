use std::time::Duration;

use component_derive::Event;
use engine::graphics::renderer::{ Renderer, Text, align };

use crate::app::assets::Assets;
use crate::world::world::World;
use crate::game_loop::*;
use crate::events::*;
use crate::app::events::*;
use crate::entities::hero::PandaType;

pub struct Game<'a> {
    pub assets: &'a Assets<'a>,
    pub world: World,
    pub level: usize,
    pub score: u32,
    pub score_this_level: u32,
    pub panda_type: PandaType,
    pub pause: f64
}
#[derive (Event)]
struct Pause(f64);

#[derive (Event)]
struct NewLevel;

impl <'a> Game<'a> {
    pub fn new(panda_type: PandaType, assets: &'a Assets<'a>, events: &mut Events) -> Game<'a> {

        let world: World = World::new(
            &assets, 
            0,
            panda_type,
            events);

        Game{ 
            assets: &assets,
            world, 
            level: 0, 
            score: 0,
            panda_type,
            score_this_level: 0,
            pause: 0.0
        }
    }
}

impl <'a> GameLoop<'a, Renderer<'a>> for Game<'a> {

    fn render(&self, renderer: &mut Renderer<'a>) -> Result<(), String> {
        self.world.render(renderer)?;
        renderer.draw_text(
            &Text { text: self.score.to_string(), justification: align::RIGHT | align::MIDDLE},
            29.0,
            19.5);
        renderer.draw_text(
            &Text { text: self.score_this_level.to_string(), justification: align::RIGHT | align::MIDDLE}, 
            3.0, 
            19.5);   
        Ok(())
    }

    fn event(&mut self, event: &Event, mut events: &mut Events) -> Result<(), String> {       
        event.apply(| score | {
            match score {
                Score::Points(p) => self.score_this_level += *p,
                Score::Double => self.score_this_level *= 2
            }
        });

        event.apply(|TimeLimitExpired| {
            events.fire(Pause(2.0));
            events.schedule(Duration::from_secs_f64(2.0), GameOver(self.score));
        });

        event.apply(|ReachedDoor| {
            self.score += self.score_this_level;
            self.score_this_level = 0;


            self.level = self.level + 1;
            if self.level < self.assets.levels.len() {
                events.fire(Pause(0.5));
                events.schedule(Duration::from_secs_f64(0.5), NewLevel);
            } else {
                events.fire(Pause(2.0));
                events.schedule(Duration::from_secs_f64(2.0), GameOver(self.score));
            }
        });

        event.apply(|NewLevel| {
            self.world = World::new(
                &self.assets,
                self.level, 
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
                return Ok(())
            }
        }
        self.world.event(event, events)
    }
}