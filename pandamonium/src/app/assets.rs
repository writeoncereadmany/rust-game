use engine::graphics::sprite::SpriteSheet;
use image::RgbImage;
use sdl2::image::LoadTexture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use std::collections::HashMap;
use std::path::PathBuf;
use tiled::{Map, PropertyValue, TileId};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct TileRef {
    pub sheet: String,
    pub tile_id: TileId,
}

pub struct TileDef {
    pub x: u32,
    pub y: u32,
    pub user_type: Option<String>,
}

pub struct TileSet(HashMap<TileRef, TileDef>);

impl TileSet {
    pub fn get(&self, tile_ref: &TileRef) -> Option<&TileDef> {
        let TileSet(tiles) = self;
        tiles.get(tile_ref)
    }
}

pub struct Level {
    pub next_level: Option<String>,
    pub next_bonus: Option<String>,
    pub layers: Vec<HashMap<(u32, u32), TileRef>>,
}

pub struct Assets<'a> {
    pub countdown: RgbImage,
    pub go: RgbImage,
    pub sheets: HashMap<String, SpriteSheet<'a>>,
    pub tiles: TileSet,
    pub levels: HashMap<String, Level>,
}

impl<'a> Assets<'a> {
    pub fn new(texture_creator: &'a TextureCreator<WindowContext>) -> Result<Self, String> {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();

        let graphics = assets.join("graphics");

        let spritesheet = texture_creator.load_texture(graphics.join("spritesheet.png"))?;
        let spritefont = texture_creator.load_texture(graphics.join("spritefont.png"))?;
        let countdown = image::open(graphics.join("countdown.png")).unwrap().to_rgb8();
        let go = image::open(graphics.join("go.png")).unwrap().to_rgb8();

        let mut sheets = HashMap::new();

        sheets.insert("Sprites".to_string(), SpriteSheet::new(spritesheet, 12, 12));
        sheets.insert("Text".to_string(), SpriteSheet::new(spritefont, 8, 8));
        let mut tiles = HashMap::new();
        let mut levels = HashMap::new();

        let mut map_loader = tiled::Loader::new();

        let mut map_files: Vec<PathBuf> = assets.join("maps").read_dir()
            .map_err(|err| format!("{err:?}"))?
            .flatten()
            .map(|dir_entry| dir_entry.path())
            .filter(|path| path.extension().map_or(false, |ext| ext == "tmx"))
            .collect();

        map_files.sort();

        for map_file in map_files {
            let map_name : String = map_file.file_stem().map(|fs| fs.to_str()).unwrap().unwrap().to_string();
            let tile_map = map_loader.load_tmx_map(map_file).map_err(|err| format!("{err:?}"))?;
            load_level(map_name, tile_map, texture_creator, &mut sheets, &mut tiles, &mut levels)?;
        }

        Ok(Assets {
            countdown,
            go,
            sheets,
            tiles: TileSet(tiles),
            levels,
        })
    }
}

fn load_level<'a>(
    map_name: String,
    tile_map: Map,
    texture_creator: &'a TextureCreator<WindowContext>,
    sheets: &mut HashMap<String, SpriteSheet<'a>>,
    tiles: &mut HashMap<TileRef, TileDef>,
    levels: &mut HashMap<String, Level>
) -> Result<(), String> {
    let next_level: Option<String> = get_string_property(&tile_map, "next_level");
    let next_bonus: Option<String> = get_string_property(&tile_map, "next_bonus");
    for tileset in tile_map.tilesets() {
        let sheet = tileset.name.to_string();

        if sheets.contains_key(&sheet) {
            continue;
        }

        let columns = tileset.columns;

        for (tile_id, tile) in tileset.tiles() {
            tiles.insert(
                TileRef { sheet: sheet.clone(), tile_id },
                TileDef { x: tile_id % columns, y: tile_id / columns, user_type: tile.user_type.clone() });
        }

        if let Some(image) = &tileset.image {
            sheets.insert(
                sheet,
                SpriteSheet::new(
                    texture_creator.load_texture(&image.source)?,
                    tileset.tile_width,
                    tileset.tile_height));
        }
    }

    let mut layers = Vec::new();

    for layer in tile_map.layers() {
        if let Some(tiles) = layer.as_tile_layer() {
            let mut map_layer = HashMap::new();
            let width = tiles.width().unwrap();
            let height = tiles.height().unwrap();
            for x in 0..width {
                for y in 0..height {
                    if let Some(tile) = tiles.get_tile(x as i32, y as i32) {
                        let sheet = tile.get_tileset().name.clone();
                        let tile_id = tile.id();

                        map_layer.insert((x, (height - 1) - y), TileRef { sheet, tile_id });
                    }
                }
            }
            layers.push(map_layer);
        }
    }

    levels.insert(map_name, Level { next_level, next_bonus, layers });
    Ok(())
}

fn get_string_property(tile_map: &Map, property: &str) -> Option<String> {
    tile_map.properties.get(property).map(|pv| match (pv) {
        PropertyValue::StringValue(val) => val.clone(),
        _ => panic!("Non-string value for {}", property)
    })
}