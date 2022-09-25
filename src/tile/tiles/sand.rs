use crate::tile::tile::*;

pub struct SandTile {}

impl Tile for SandTile {
    fn get_type(&self) -> TileType {
        return TileType::SAND;
    }

    fn get_texture_path(&self) -> String {
        return String::from("assets/sand.png");
    }

    fn get_hardness(&self) -> i32 {
        return 0;
    }
}
