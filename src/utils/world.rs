use perlin2d::PerlinNoise2D;
use sdl2::render::Canvas;
use sdl2::EventPump;

use crate::game::gamestate::GAME;
use crate::gfx::assetmanager::AssetManager;
use crate::tile::tile::WorldTile;
use crate::tile::tiles::grass::*;
use crate::tile::tiles::sand::*;
use crate::tile::tiles::water::*;
use crate::world::world::World;

pub fn generate_overworld(world: &mut World, seed: i32) {
    let perlin = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, seed);

    for x in 0..1000 {
        for y in 0..1000 {
            let noise = perlin.get_noise(x as f64, y as f64);

            if noise < -6.0 {
                world.insert_tile(WorldTile::from_tile(x * 32, y * 32, WaterTile {}));
            } else if noise < -4.0 {
                world.insert_tile(WorldTile::from_tile(x * 32, y * 32, SandTile {}));
            } else {
                world.insert_tile(WorldTile::from_tile(x * 32, y * 32, GrassTile {}));
            }
        }
    }
}

pub fn render_overworld(
    event_pump: &mut EventPump,
    asset_manager: &AssetManager,
    canvas: &mut Canvas<sdl2::video::Window>,
) {
    let mut overworld = GAME.lock().overworld.clone();
    overworld.update_entitys(event_pump);
    overworld.render(canvas, asset_manager);
}
