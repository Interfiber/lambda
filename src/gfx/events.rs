use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Point;
use sdl2::EventPump;

use crate::entity::entity::Direction;
use crate::game::gamestate::GAME;

pub fn key_events(event_pump: &mut EventPump) {
    for event in event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => {
                info!("Shutting down");
                std::process::exit(0);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::NORTH;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::SOUTH;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::WEST;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::EAST;
            }
            Event::KeyUp {
                keycode: Some(Keycode::Up),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Down),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Left),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::Right),
                ..
            } => {
                GAME.lock().player_speed = 0;
            }
            _ => {}
        }
    }
}

pub fn mouse_events(event_pump: &EventPump) {
    let mouse_state = event_pump.mouse_state();
    if mouse_state.left() {
        info!("X: {}, Y: {}", mouse_state.x(), mouse_state.y());
        GAME.lock().player_position = Point::new(mouse_state.x(), mouse_state.y());
    }
}
