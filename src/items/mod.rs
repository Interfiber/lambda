pub mod itemloader;
pub mod items;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum ItemType {
    Wood = 0,
}

// on disk info info
#[derive(Deserialize, Serialize, Debug)]
pub struct ItemInfo {
    pub item_type: ItemType,
    pub item_texture: String,
}
