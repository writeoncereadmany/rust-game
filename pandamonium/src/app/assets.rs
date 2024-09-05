use image::RgbImage;
use std::collections::HashMap;
use std::path::PathBuf;
use engine::graphics::sprite::SpriteSheet;
use sdl2::image::LoadTexture;
use sdl2::render::{Texture, TextureCreator};
use sdl2::video::WindowContext;
use tiled::TileId;

pub struct Assets<'a> {
    pub countdown: RgbImage,
    pub go: RgbImage,
    pub sheets : HashMap<String, SpriteSheet<'a>>,
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

        sheets.insert("Sprites".to_string(), SpriteSheet::new(spritesheet, 12, 12));
        sheets.insert("Text".to_string(), SpriteSheet::new(spritefont, 8, 8));
        let mut tiles = HashMap::new();
        let mut map = Vec::new();

        let mut map_loader = tiled::Loader::new();
        let tile_map = map_loader.load_tmx_map(assets.join("maps").join("001.tmx")).map_err(|err| format!("{err:?}"))?;
        for (index, tileset) in tile_map.tilesets().iter().enumerate() {

            let tileset_name = tileset.name.to_string();

            let columns = tileset.columns;

            for (tile_id, tile) in tileset.tiles() {
                tiles.insert((index, tile_id), (tileset_name.clone(), (tile_id % columns, tile_id / columns), tile.user_type.clone()));
            }

            if let Some(image) = &tileset.image {
                sheets.insert(
                    tileset_name,
                    SpriteSheet::new(
                        texture_creator.load_texture(&image.source)?,
                        tileset.tile_width,
                        tileset.tile_height));
            }
        }

        for layer in tile_map.layers() {
            if let Some(tiles) = layer.as_tile_layer() {
                let mut map_layer = HashMap::new();
                let width = tiles.width().unwrap();
                let height = tiles.height().unwrap();
                for x in 0..width {
                    for y in 0..height {
                        if let Some(tile) = tiles.get_tile(x as i32, y as i32) {
                            map_layer.insert((x, (height - 1) - y), (tile.tileset_index(), tile.id()));
                        }
                    }
                }
                map.push(map_layer);
            }
        }

        Ok(Assets {
            countdown,
            go,
            sheets,
            tiles,
            map
        })
    }
}