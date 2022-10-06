use lazy_static::lazy_static;
use parking_lot::Mutex;
use sdl2::rect::Point;

use crate::{
    entity::entity::Direction,
    world::world::{World, WorldTemplate},
};

pub struct GameState {
    pub player_facing: Direction,
    pub player_position: Point,
    pub player_speed: i32,
    pub player_static_speed: i32, // not modified, used during speed restore
    pub camera: sdl2::rect::Rect,
    pub screen_w: i32,
    pub screen_h: i32,
    pub fps: u32,
    pub overworld: World,
    pub tile_size: i32,
}

impl GameState {
    pub fn set_player_position(&mut self, v: Point) {
        self.player_position = v;
    }

    pub fn restore_speed(&mut self) {
        self.player_speed = self.player_static_speed;
    }

    pub fn get_position_infront_of_player(&self) -> Point {
        match self.player_facing {
            Direction::North => return self.player_position.offset(0, -self.tile_size),
            Direction::South => return self.player_position.offset(0, self.tile_size),
            Direction::East => return self.player_position.offset(self.tile_size, 0),
            Direction::West => return self.player_position.offset(-self.tile_size, 0),
        }
    }
}

pub fn new_gamestate() -> GameState {
    GameState {
        player_facing: Direction::North,
        player_position: Point::new(100, 100),
        camera: sdl2::rect::Rect::new(0, 0, 1000, 1000),
        screen_h: 0,
        screen_w: 0,
        player_speed: 0,
        player_static_speed: 3,
        fps: 0,
        overworld: WorldTemplate::create(),
        tile_size: 32,
    }
}

lazy_static! {
    pub static ref GAME: Mutex<GameState> = Mutex::new(new_gamestate());
}
