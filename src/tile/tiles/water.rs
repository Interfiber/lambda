use crate::tile::tile::*;

pub struct WaterTile {}

impl Tile for WaterTile {
    fn get_type(&self) -> TileType {
        return TileType::WATER;
    }

    fn get_texture_path(&self) -> String {
        return String::from("assets/water.png");
    }

    fn get_hardness(&self) -> i32 {
        return 0;
    }
}
