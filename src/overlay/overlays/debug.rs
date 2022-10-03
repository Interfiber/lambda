use sdl2::{rect::Rect, render::Canvas};

use crate::{
    game::gamestate::GAME,
    gfx::{
        assetmanager::AssetManager,
        fontmanager::{FontDetails, FontManager},
    },
    overlay::overlay::Overlay,
};

pub struct DebugOverlay {}

impl Overlay for DebugOverlay {
    fn render_overlay(
        &self,
        canvas: &mut Canvas<sdl2::video::Window>,
        _asset_manager: &AssetManager,
        font_manager: &mut FontManager,
    ) {
        let font = font_manager
            .load(&FontDetails {
                path: "assets/LanaPixel.ttf".to_string(),
                size: 10,
            })
            .expect("Failed to load font");

        let font_info = crate::utils::font::render_font_to_texture(
            &font,
            &canvas.texture_creator(),
            &format!("FPS: {}", GAME.lock().fps),
        );

        let pos = GAME.lock().player_position;

        let font_info_2 = crate::utils::font::render_font_to_texture(
            &font,
            &canvas.texture_creator(),
            &format!("Position: {}, {}", pos.x, pos.y),
        );

        let rect = Rect::new(0, 0, font_info.0, font_info.1);

        canvas.copy(&font_info.2, None, rect).expect("Copy failed!");
        canvas
            .copy(
                &font_info_2.2,
                None,
                Rect::new(0, font_info.1 as i32, font_info_2.0, font_info_2.1),
            )
            .expect("Copy failed!");

        unsafe {
            font_info.2.destroy();
            font_info_2.2.destroy();
        }
    }
}
