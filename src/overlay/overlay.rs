use sdl2::render::Canvas;

use crate::gfx::{assetmanager::AssetManager, fontmanager::FontManager};

pub trait Overlay {
    fn render_overlay(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        asset_manager: &AssetManager,
        font_manager: &mut FontManager,
    );
}
