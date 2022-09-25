use crate::gfx::assetmanager::*;

pub fn bootstrap() {
    info!("Bootstrapping lambda v2.0");

    let mut asset_manager = AssetManagerTemplate::new();

    let assets = vec![
        "assets/water.png".to_string(),
        "assets/grass.png".to_string(),
        "assets/sand.png".to_string(),
        "assets/player_walk_up.png".to_string(),
        "assets/player_walk_down.png".to_string(),
        "assets/player_walk_left.png".to_string(),
        "assets/player_walk_right.png".to_string(),
    ];

    info!("Creating asset manager queue");
    asset_manager.new_queue(assets);

    info!("Running GFX bootstrap");
    crate::bootstrap::gfxbootstrap::bootstrap(asset_manager);
}
