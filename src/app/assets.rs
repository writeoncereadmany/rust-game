use image::RgbImage;

use sdl2::rect::Rect;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

use crate::graphics::sprite::Sprite;
use crate::graphics::text_renderer::SpriteFont;

pub struct Assets<'a> {
    tile_width: u32,
    tile_height: u32,
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
            tile_width: 12,
            tile_height: 12,
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

    pub fn sprite(&'a self, x: i32, y: i32) -> Sprite<'a> {
        Sprite::new(
            &self.spritesheet, 
            Rect::new(
                x * self.tile_width as i32, 
                y * self.tile_height as i32, 
                self.tile_width, 
                self.tile_height
            )
        )
    }

    pub fn multisprite(&'a self, x: i32, y: i32, width: u32, height: u32) -> Sprite<'a> {
        Sprite::new(
            &self.spritesheet, 
            Rect::new(
                x * self.tile_width as i32, 
                y * self.tile_height as i32, 
                self.tile_width * width, 
                self.tile_height * height
            )
        )
    }
}