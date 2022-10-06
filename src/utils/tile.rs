use crate::tile::tile::WorldTile;
use sdl2::rect::Rect;

use crate::{game::gamestate::GAME, tile::tile::Tile};

pub fn get_tile_rect(x: i32, y: i32) -> Rect {
    let size = GAME.lock().tile_size;
    Rect::new(x, y, size as u32, size as u32)
}

pub fn place_tile_as_player<T>(tile: T)
where
    T: Tile,
{
    let pos = GAME.lock().get_position_infront_of_player();
    let tile_size = GAME.lock().tile_size;

    let final_x = ((pos.x / tile_size) as f32).floor() as i32;
    let final_y = ((pos.y / tile_size) as f32).floor() as i32;
    GAME.lock().overworld.insert_tile(WorldTile::from_tile(
        final_x * tile_size,
        final_y * tile_size,
        tile,
    ));
}
