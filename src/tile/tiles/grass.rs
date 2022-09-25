use crate::tile::tile::*;

pub struct GrassTile {}

impl Tile for GrassTile {
    fn get_type(&self) -> TileType {
        return TileType::GRASS;
    }

    fn get_texture_path(&self) -> String {
        return String::from("assets/grass.png");
    }

    fn get_hardness(&self) -> i32 {
        return 0;
    }
}
