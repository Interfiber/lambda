use super::entitys::player::PlayerEntity;

#[derive(Clone)]
pub struct SpawnedEntity {
    pub entity_type: EntityType,
    pub x: f32,
    pub y: f32,
}

impl SpawnedEntity {
    pub fn from_entity<T>(entity: T) -> SpawnedEntity
    where
        T: Entity,
    {
        return SpawnedEntity {
            x: entity.get_x(),
            y: entity.get_y(),
            entity_type: entity.get_type(),
        };
    }

    pub fn to_entity(&self) -> Box<dyn Entity> {
        match self.entity_type {
            EntityType::PLAYER => return Box::new(PlayerEntity {}),
        }
    }]
}

pub trait Entity {
    fn update(&self, event_pump: &sdl2::EventPump); // called every frame after entity is rendered
    fn get_texture_path(&self) -> String;
    fn get_texture_height(&self) -> i32;
    fn get_texture_width(&self) -> i32;
    fn get_type(&self) -> EntityType;
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

#[derive(Clone)]
pub enum Direction {
    NORTH, // up
    SOUTH, // down
    EAST,  // right
    WEST,  // left
}

#[derive(Clone)]
pub enum EntityType {
    PLAYER,
}
