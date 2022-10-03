use crate::tile::tile::*;

pub struct SandTile {}

impl Tile for SandTile {
    fn get_type(&self) -> TileType {
        TileType::Sand
    }

    fn get_texture_path(&self) -> String {
        String::from("assets/sand.png")
    }

    fn get_hardness(&self) -> i32 {
        0
    }
}
