use lazy_static::lazy_static;
use parking_lot::Mutex;
use sdl2::rect::Point;

use crate::entity::entity::Direction;

#[derive(Debug)]
pub struct GameState {
    pub player_facing: Direction,
    pub player_position: Point,
    pub player_speed: i32,
    pub player_static_speed: i32, // not modified, used during speed restore
    pub camera: sdl2::rect::Rect,
    pub screen_w: i32,
    pub screen_h: i32,
    pub fps: u32,
}

impl GameState {
    pub fn set_player_position(&mut self, v: Point) {
        self.player_position = v;
    }

    pub fn restore_speed(&mut self) {
        self.player_speed = self.player_static_speed;
    }
}

pub fn new_gamestate() -> GameState {
    return GameState {
        player_facing: Direction::NORTH,
        player_position: Point::new(100, 100),
        camera: sdl2::rect::Rect::new(0, 0, 1000, 1000),
        screen_h: 0,
        screen_w: 0,
        player_speed: 0,
        player_static_speed: 3,
        fps: 0,
    };
}

lazy_static! {
    pub static ref GAME: Mutex<GameState> = Mutex::new(new_gamestate());
}
