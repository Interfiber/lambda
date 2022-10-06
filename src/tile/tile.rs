use super::tiles::{grass::GrassTile, sand::SandTile, water::WaterTile, wood::WoodTile};

pub trait Tile {
    fn get_type(&self) -> TileType;
    fn get_texture_path(&self) -> String;
    fn get_hardness(&self) -> i32;
}

#[derive(Clone, Copy)]
pub struct WorldTile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

impl WorldTile {
    pub fn from_tile<T>(x: i32, y: i32, tile: T) -> WorldTile
    where
        T: Tile,
    {
        WorldTile {
            x,
            y,
            tile_type: tile.get_type(),
        }
    }

    pub fn to_tile(&self) -> Box<dyn Tile> {
        match self.tile_type {
            TileType::Sand => Box::new(SandTile {}),
            TileType::Grass => Box::new(GrassTile {}),
            TileType::Water => Box::new(WaterTile {}),
            TileType::Wood => Box::new(WoodTile {}),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TileType {
    Grass,
    Sand,
    Water,
    Wood,
}
