use lazy_static::lazy_static;

lazy_static! {
    pub static ref WOOD_ITEM: String =
        String::from_utf8_lossy(include_bytes!("item_info/wood.ron")).to_string();
}

pub fn get_item_data() -> Vec<String> {
    vec![WOOD_ITEM.to_string()]
}
