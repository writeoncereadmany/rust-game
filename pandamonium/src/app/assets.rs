use std::path::PathBuf;

use image::RgbImage;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;

pub struct Assets<'a> {
    pub spritesheet : Texture<'a>,
    pub spritefont: Texture<'a>,
    pub levels: Vec<RgbImage>,
    pub countdown: RgbImage,
    pub go: RgbImage,

}

impl <'a> Assets<'a> {
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

        let graphics = assets.join("graphics");

        let spritesheet = texture_creator.load_texture(graphics.join("spritesheet.png"))?;
        let spritefont = texture_creator.load_texture(graphics.join("spritefont.png"))?;
        let countdown = image::open(graphics.join("countdown.png")).unwrap().to_rgb8();
        let go = image::open(graphics.join("go.png")).unwrap().to_rgb8();


        let mut levels: Vec<PathBuf> = assets.join("levels").read_dir()
            .map_err(|_e| "Failed")?
            .map(|result| result.unwrap().path())
            .collect();

        levels.sort();
        let levels: Vec<RgbImage> = levels.iter()    
            .map(|file| { image::open(file).unwrap().to_rgb8() })
            .collect();
        

        Ok(Assets {
            spritesheet,
            spritefont,
            levels,
            countdown,
            go
        })
    }
}