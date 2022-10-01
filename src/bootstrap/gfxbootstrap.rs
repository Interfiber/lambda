use crate::gfx::window::WindowTemplate;

pub fn bootstrap(assets: Vec<String>) {
    info!("Bootstrapping GFX");

    let mut window = WindowTemplate::new("Lambda - SDL2");
    window.load_assets_on_render(assets);
    window.set_fullscreen(true);

    info!("Starting game loop");
    window.render();
}
