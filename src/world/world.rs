use sdl2::{
    rect::{Point, Rect},
    render::Canvas,
};

use crate::{
    entity::entity::SpawnedEntity, gfx::assetmanager::AssetManager, tile::tile::WorldTile,
};
use sdl2::video::Window;

pub struct World {
    world_data: Vec<WorldTile>,
    entity_data: Vec<SpawnedEntity>,
}

pub struct WorldTemplate {}

impl WorldTemplate {
    pub fn new() -> World {
        return World {
            world_data: vec![],
            entity_data: vec![],
        };
    }
}

impl World {
    pub fn insert_tile(&mut self, tile: WorldTile) {
        self.world_data.push(tile);
    }

    pub fn spawn_entity(&mut self, entity: SpawnedEntity) {
        self.entity_data.push(entity);
    }

    pub fn render(&self, canvas: &mut Canvas<Window>, asset_manager: &AssetManager) {
        let rect = Rect::new(0, 0, 1000, 1000);

        // render world
        for tile in self.world_data.to_vec().into_iter() {
            let point = Point::new(tile.x, tile.y);
            if rect.contains_point(point) {
                canvas
                    .copy(
                        asset_manager.get_texture(tile.to_tile().get_texture_path()),
                        None,
                        crate::tile::utils::get_tile_rect(tile.x, tile.y),
                    )
                    .expect("Copy failed");
            }
        }

        // render entitys
        for entity in self.entity_data.to_vec().into_iter() {
            let entity_info = entity.to_entity();
            let rect = Rect::new(
                entity.x as i32,
                entity.y as i32,
                entity_info.get_texture_width() as u32,
                entity_info.get_texture_height() as u32,
            );
            canvas
                .copy(
                    asset_manager.get_texture(entity_info.get_texture_path()),
                    None,
                    rect,
                )
                .expect("Copy failed");
        }
    }

    pub fn update_entitys(&mut self, event_pump: &sdl2::EventPump) {
        let mut i = 0;
        for mut entity in self.entity_data.to_vec().into_iter() {
            let entity_info = entity.to_entity();
            entity_info.update(event_pump);

            entity.x = entity_info.get_x();
            entity.y = entity_info.get_y();

            self.entity_data.remove(i);
            self.entity_data.push(entity);
            i += 1;
        }
    }
}
