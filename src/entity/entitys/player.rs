use crate::{
    entity::entity::{Direction, Entity, EntityType},
    game::gamestate::GAME,
};
use sdl2::keyboard::Scancode;
pub struct PlayerEntity {}

impl Entity for PlayerEntity {
    fn update(&self, event_pump: &sdl2::EventPump) {
        let mut x = GAME.lock().unwrap().player_x;
        let mut y = GAME.lock().unwrap().player_y;
        if event_pump.keyboard_state().is_scancode_pressed(Scancode::A) {
            GAME.lock().unwrap().player_facing = Direction::WEST;
            x -= 5.0;
        }

        if event_pump.keyboard_state().is_scancode_pressed(Scancode::D) {
            GAME.lock().unwrap().player_facing = Direction::EAST;
            x += 5.0;
        }

        if event_pump.keyboard_state().is_scancode_pressed(Scancode::W) {
            GAME.lock().unwrap().player_facing = Direction::NORTH;
            y -= 5.0;
        }

        if event_pump.keyboard_state().is_scancode_pressed(Scancode::S) {
            GAME.lock().unwrap().player_facing = Direction::SOUTH;
            y += 5.0;
        }

        GAME.lock().unwrap().set_player_x(x);
        GAME.lock().unwrap().set_player_y(y);
    }

    fn get_texture_path(&self) -> String {
        match GAME.lock().unwrap().player_facing {
            Direction::NORTH => return String::from("assets/player_walk_up.png"),
            Direction::SOUTH => return String::from("assets/player_walk_down.png"),
            Direction::EAST => return String::from("assets/player_walk_right.png"),
            Direction::WEST => return String::from("assets/player_walk_left.png"),
        }
    }

    fn get_texture_height(&self) -> i32 {
        return 16;
    }

    fn get_texture_width(&self) -> i32 {
        return 16;
    }

    fn get_type(&self) -> EntityType {
        return EntityType::PLAYER;
    }

    fn get_x(&self) -> f32 {
        return GAME.lock().unwrap().player_x;
    }

    fn get_y(&self) -> f32 {
        return GAME.lock().unwrap().player_y;
    }
}
