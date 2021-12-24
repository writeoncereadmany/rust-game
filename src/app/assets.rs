use image::RgbImage;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Assets<'a> {
    pub spritesheet : Texture<'a>,
    pub tilesheet : Texture<'a>,
    pub numbersheet: Texture<'a>,
    pub spritefont: Texture<'a>,
    pub level: RgbImage,
}

impl <'a> Assets<'a> {
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

        let graphics = assets.join("graphics");

        let spritesheet = texture_creator.load_texture(graphics.join("ball.png"))?;
        let tilesheet = texture_creator.load_texture(graphics.join("12x12tile.png"))?;
        let numbersheet = texture_creator.load_texture(graphics.join("numbers.png"))?;
        let spritefont = texture_creator.load_texture(graphics.join("spritefont.png"))?;

        let levels = assets.join("levels");
        let level : RgbImage = image::open(levels.join("level1.png")).unwrap().to_rgb8();

        Ok(Assets {
            spritesheet,
            tilesheet,
            numbersheet,
            spritefont,
            level,
        })
    }
}