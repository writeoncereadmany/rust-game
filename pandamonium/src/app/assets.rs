use image::RgbImage;
use std::collections::HashMap;
use std::path::PathBuf;

use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use tiled::TileId;

pub struct Assets<'a> {
    pub spritesheet : Texture<'a>,
    pub spritefont: Texture<'a>,
    pub levels: Vec<RgbImage>,
    pub countdown: RgbImage,
    pub go: RgbImage,
    pub sheets : HashMap<String, Texture<'a>>,
    pub tiles : HashMap<(usize, TileId), (String, (u32, u32), Option<String>)>,
    pub map : Vec<HashMap<(u32, u32), (usize, TileId)>>
}

impl <'a> Assets<'a> {
    pub fn new(texture_creator : &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let assets = find_folder::Search::ParentsThenKids(3,3).for_folder("assets").unwrap();

        let graphics = assets.join("graphics");

        let spritesheet = texture_creator.load_texture(graphics.join("spritesheet.png"))?;
        let spritefont = texture_creator.load_texture(graphics.join("spritefont.png"))?;
        let countdown = image::open(graphics.join("countdown.png")).unwrap().to_rgb8();
        let go = image::open(graphics.join("go.png")).unwrap().to_rgb8();

        let mut sheets = HashMap::new();
        let mut tiles = HashMap::new();
        let mut map = Vec::new();

        let mut map_loader = tiled::Loader::new();
        let tile_map = map_loader.load_tmx_map(assets.join("maps").join("001.tmx")).map_err(|err| format!("{err:?}"))?;
        for (index, tileset) in tile_map.tilesets().iter().enumerate() {

            let name = tileset.name.to_string();

            let columns = tileset.columns;

            for (tile_id, tile) in tileset.tiles() {
                tiles.insert((index, tile_id), (name.clone(), (tile_id % columns, tile_id / columns), tile.user_type.clone()));
            }

            if let Some(image) = &tileset.image {
                sheets.insert(name, texture_creator.load_texture(&image.source)?);
            }
        }

        for layer in tile_map.layers() {
            if let Some(tiles) = layer.as_tile_layer() {
                let mut map_layer = HashMap::new();
                for x in 0..tiles.width().unwrap() {
                    for y in 0..tiles.height().unwrap() {
                        if let Some(tile) = tiles.get_tile(x as i32, y as i32) {
                            map_layer.insert((x, y), (tile.tileset_index(), tile.id()));
                        }
                    }
                }
                map.push(map_layer);
            }
        }


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
            go,
            sheets,
            tiles,
            map
        })
    }
}