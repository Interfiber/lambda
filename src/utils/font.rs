use sdl2::{
    pixels::Color,
    render::{Texture, TextureCreator},
    ttf::Font,
    video::WindowContext,
};

pub fn render_font_to_texture(
    font: &Font,
    texture_creator: &TextureCreator<WindowContext>,
    text: &str,
) -> (u32, u32, Texture) {
    let surface = font
        .render(text)
        .blended(Color::RGBA(255, 255, 255, 0))
        .map_err(|e| e.to_string())
        .expect("Failed to render font to surface!");

    let font_texture = texture_creator
        .create_texture_from_surface(&surface)
        .map_err(|e| e.to_string())
        .expect("Failed to create texture from surface!");

    (surface.width(), surface.height(), font_texture)
}
