use image::RgbImage;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::graphics::text_renderer::SpriteFont;

pub struct Assets<'a> {
    char_width: u32,
    char_height: u32,
    pub spritesheet : Texture<'a>,
    pub spritefont: Texture<'a>,
    pub level: Vec<RgbImage>,
}

impl <'a> Assets<'a> {
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

        let graphics = assets.join("graphics");

        let spritesheet = texture_creator.load_texture(graphics.join("spritesheet.png"))?;
        let spritefont = texture_creator.load_texture(graphics.join("spritefont.png"))?;

        let levels = assets.join("levels");
        let level : Vec<RgbImage> = ["level0.png", "level1.png"]
            .iter()
            .map(|file| { image::open(levels.join(file)).unwrap().to_rgb8() })
            .collect();

        Ok(Assets {
            char_width: 8,
            char_height: 8,
            spritesheet,
            spritefont,
            level,
        })
    }

    pub fn spritefont(&'a self) -> SpriteFont<'a> {
        SpriteFont::new(&self.spritefont, self.char_width, self.char_height)
    }
}