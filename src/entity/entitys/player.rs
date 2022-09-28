use crate::{
    entity::entity::{Direction, Entity, EntityType},
    game::gamestate::GAME,
};
use sdl2::rect::Point;

pub struct PlayerEntity {}

impl Entity for PlayerEntity {
    fn update(&self, _event_pump: &mut sdl2::EventPump) {
        let x = GAME.lock().player_position.x;
        let y = GAME.lock().player_position.y;
        let mut pos = Point::new(x, y);
        let speed = GAME.lock().player_speed;
        let facing = GAME.lock().player_facing;

        match facing {
            Direction::NORTH => {
                pos = pos.offset(0, -speed);
            }
            Direction::SOUTH => {
                pos = pos.offset(0, speed);
            }
            Direction::EAST => {
                pos = pos.offset(speed, 0);
            }
            Direction::WEST => {
                pos = pos.offset(-speed, 0);
            }
        }
        if GAME.is_locked(){
            error!("Game object locked!");
        } else {
            GAME.lock().set_player_position(pos);
        }
    }

    fn get_texture_path(&self) -> String {
        match GAME.lock().player_facing {
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

    fn get_position(&self) -> sdl2::rect::Point {
        let x = GAME.lock().player_position.x;
        let y = GAME.lock().player_position.y;

        return Point::new(x, y);
    }
}
