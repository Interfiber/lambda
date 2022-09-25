use sdl2::rect::Rect;

pub fn get_tile_rect(x: i32, y: i32) -> Rect {
    return Rect::new(x, y, get_tile_size(), get_tile_size());
}

pub fn get_tile_size() -> u32 {
    return 32;
}
