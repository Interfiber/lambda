pub fn bootstrap() {
    info!("Bootstrapping lambda v2.0");

    let assets = vec![
        "assets/water.png".to_string(),
        "assets/grass.png".to_string(),
        "assets/sand.png".to_string(),
        "assets/player_walk_up.png".to_string(),
        "assets/player_walk_down.png".to_string(),
        "assets/player_walk_left.png".to_string(),
        "assets/player_walk_right.png".to_string(),
    ];

    info!("Running GFX bootstrap");
    crate::bootstrap::gfxbootstrap::bootstrap(assets);
}
