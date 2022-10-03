use sdl2::event::Event;
use sdl2::keyboard::Keycode;
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
                keycode: Some(Keycode::W),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::North;
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::South;
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::West;
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                GAME.lock().restore_speed();
                GAME.lock().player_facing = Direction::East;
            }
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            }
            | Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => {
                GAME.lock().player_speed = 0;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => {
                info!("Place tile");
            }
            _ => {}
        }
    }
}
