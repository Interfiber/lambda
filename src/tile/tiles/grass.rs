use crate::tile::tile::*;

pub struct GrassTile {}

impl Tile for GrassTile {
    fn get_type(&self) -> TileType {
        TileType::Grass
    }

    fn get_texture_path(&self) -> String {
        String::from("assets/grass.png")
    }

    fn get_hardness(&self) -> i32 {
        0
    }
}
