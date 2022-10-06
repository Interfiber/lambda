use crate::items::{ItemInfo, ItemType};

pub struct ItemLoader {
    pub textures: Vec<String>,
}

pub struct ItemLoaderTemplate {}

impl ItemLoaderTemplate {
    pub fn new() -> ItemLoader {
        ItemLoader { textures: vec![] }
    }
}

impl ItemLoader {
    pub fn load_items(&mut self) {
        info!("Loading item info from disk");

        let item_data = crate::items::items::get_item_data();

        let size = item_data.len();
        let mut i = 0;
        for item in item_data.into_iter() {
            let item_info: ItemInfo =
                ron::from_str(&item).expect("Failed to parse item data, THIS IS A BUG!!!");
            self.textures
                .insert(self.textures.len(), item_info.item_texture);

            info!("Loaded item info: {:?} [{}/{}]", item_info.item_type, i, size);
            i += 1;
        }
    }
}
