pub fn bootstrap() {
    info!("Bootstrapping lambda v2.0");

    info!("Loading items...");

    let mut item_loader = crate::items::itemloader::ItemLoaderTemplate::new();
    item_loader.load_items();

    let mut assets = vec![
        "assets/water.png".to_string(),
        "assets/grass.png".to_string(),
        "assets/sand.png".to_string(),
        "assets/player_walk_up.png".to_string(),
        "assets/player_walk_down.png".to_string(),
        "assets/player_walk_left.png".to_string(),
        "assets/player_walk_right.png".to_string(),
        "assets/wood_planks.png".to_string(),
    ];

    info!("Appending item loader textures into asset manager queue");

    assets.append(&mut item_loader.textures);

    info!("Running GFX bootstrap");
    crate::bootstrap::gfxbootstrap::bootstrap(assets);
}
