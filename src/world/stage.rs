use image::{Rgb, RgbImage};

use crate::map::Map;

pub fn from_image<A, F>(map: &mut Map<A>, image: &RgbImage, tiler: F) where A: Clone, F: Fn(&Rgb<u8>) -> Option<A> {
    let height = image.height();
    for x in 0..image.width() {
        for y in 0..height {
            let pixel: &Rgb<u8> = image.get_pixel(x, height - 1 - y);
            match tiler(pixel) {
                Some(tile) => { map.put(x as i32, y as i32, tile); },
                None => { }
            }
            
        }
    }
}