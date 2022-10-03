use crate::tile::tile::*;

pub struct WaterTile {}

impl Tile for WaterTile {
    fn get_type(&self) -> TileType {
        TileType::Water
    }

    fn get_texture_path(&self) -> String {
        String::from("assets/water.png")
    }

    fn get_hardness(&self) -> i32 {
        0
    }
}
