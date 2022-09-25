use std::sync::Mutex;

use lazy_static::lazy_static;

use crate::entity::entity::Direction;

pub struct GameState {
    pub player_facing: Direction,
    pub player_x: f32,
    pub player_y: f32,
}

impl GameState {
    pub fn set_player_x(&mut self, v: f32) {
        self.player_x = v;
    }

    pub fn set_player_y(&mut self, v: f32) {
        self.player_y = v;
    }
}

pub fn new_gamestate() -> GameState {
    return GameState {
        player_facing: Direction::NORTH,
        player_x: 20.0,
        player_y: 20.0,
    };
}

lazy_static! {
    pub static ref GAME: Mutex<GameState> = Mutex::new(new_gamestate());
}
