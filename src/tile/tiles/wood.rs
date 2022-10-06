use crate::tile::tile::*;

pub struct WoodTile {}

impl Tile for WoodTile {
    fn get_type(&self) -> TileType {
        TileType::Wood
    }

    fn get_texture_path(&self) -> String {
        String::from("assets/wood_planks.png")
    }

    fn get_hardness(&self) -> i32 {
        0
    }
}
