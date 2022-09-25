use crate::gfx::{assetmanager::AssetManager, window::WindowTemplate};

pub fn bootstrap(asset_manager: AssetManager) {
    info!("Bootstrapping GFX");

    let mut window = WindowTemplate::new("LambdaSDL_RUST");
    window.load_assets_on_render(asset_manager);
    window.set_fullscreen(true);

    info!("Starting game loop");
    window.render();
}
