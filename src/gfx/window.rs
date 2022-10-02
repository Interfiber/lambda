use std::time::SystemTime;

use crate::entity::entity::SpawnedEntity;
use crate::entity::entitys::player::PlayerEntity;
use crate::game::gamestate::GAME;
use crate::gfx::fontmanager::{FontDetails, FontManager};
use sdl2::gfx::framerate::FPSManager;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::WindowContext;

use crate::world::world::WorldTemplate;

use super::assetmanager::{AssetManager, AssetManagerTemplate};

pub struct WindowTemplate {
    pub title: String,
}

pub struct Window {
    pub title: String,
    pub texture_creator: Option<TextureCreator<WindowContext>>,
    pub load_assets: bool,
    pub asset_manager: Option<AssetManager>,
    pub fullscreen: bool, // TODO make fullscreen work
}

impl WindowTemplate {
    pub fn new(title: &str) -> Window {
        return Window {
            title: title.to_string(),
            texture_creator: Default::default(),
            load_assets: false,
            asset_manager: Some(AssetManagerTemplate::new()),
            fullscreen: false,
        };
    }
}

impl Window {
    pub fn render(&mut self) {
        info!("Creating sdl context");
        let sdl_context = sdl2::init().expect("Failed to init SDL2");
        info!("Creating video subsystem");
        let video_subsystem = sdl_context.video().expect("Failed to get video subsystem");
        info!("Creating ttf font subsystem");
        let font_context = sdl2::ttf::init()
            .map_err(|e| e.to_string())
            .expect("Failed to init ttf");

        info!("Creating window using sdl2 with OpenGL rendering");
        let window = video_subsystem
            .window(&self.title, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create window");

        info!("Creating sdl2 canvas");
        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create SDL2 canvas");
        self.texture_creator = Some(canvas.texture_creator());

        if self.load_assets {
            info!("Requesting asset manager to load assets from queue");
            self.asset_manager
                .as_mut()
                .unwrap()
                .load_assets_from_queue(self.texture_creator.as_ref().unwrap());
            debug!("Clearing asset manager queue");
            self.asset_manager.as_mut().unwrap().clear_queue();
        }

        let mut font_manager = FontManager::new(&font_context);

        canvas.set_draw_color(Color::GREY);
        canvas.clear();
        canvas.present();

        info!("Creating event pump");
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event punp");

        let mut world = WorldTemplate::new();

        info!("Generating world");
        let seed = SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Failed to get time since unix epoch");
        crate::utils::world::generate_overworld(&mut world, seed.as_millis());

        world.spawn_entity(SpawnedEntity::from_entity(PlayerEntity {}));
        info!("World generated");
        GAME.lock().overworld = world;

        canvas.set_scale(4.0, 4.0).expect("Failed to set scale");

        let mut fps = FPSManager::new();
        fps.set_framerate(60).expect("Framerate set failed");

        info!("Starting game loop");

        loop {
            crate::gfx::events::key_events(&mut event_pump);

            let old = sdl_context.timer().unwrap().ticks();
            Self::ajust_screen(&canvas);

            canvas.clear();
            crate::utils::world::render_overworld(&mut event_pump, self.asset_manager.as_ref().unwrap(), &mut canvas);
            let font = font_manager
                .load(&FontDetails {
                    path: "assets/LanaPixel.ttf".to_string(),
                    size: 10,
                })
                .expect("Failed to load font");

            let font_info = crate::utils::font::render_font_to_texture(
                &font,
                &self.texture_creator.as_ref().unwrap(),
                &format!("FPS: {}", GAME.lock().fps.to_string()),
            );

            let pos = GAME.lock().player_position;

            let font_info_2 = crate::utils::font::render_font_to_texture(
                &font,
                &self.texture_creator.as_ref().unwrap(),
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

            canvas.present();

            fps.delay();
            let new = sdl_context.timer().unwrap().ticks();

            GAME.lock().fps = 1000 / (new - old);
            // info!("Deltatime: {}", (new - old) as f32 * 0.4);
        }
    }

    pub fn load_assets_on_render(&mut self, queue: Vec<String>) {
        self.load_assets = true;
        self.asset_manager.as_mut().unwrap().new_queue(queue);
    }

    pub fn set_fullscreen(&mut self, enabled: bool) {
        self.fullscreen = enabled;
    }

    pub fn ajust_screen(canvas: &Canvas<sdl2::video::Window>) {
        let screen_w = GAME.lock().screen_w;
        let screen_h = GAME.lock().screen_h;

        if screen_w != canvas.output_size().unwrap().0 as i32
            || screen_h != canvas.output_size().unwrap().1 as i32
        {
            GAME.lock().camera.x = 0;
            GAME.lock().camera.y = 0;
        } else {
            let player_x = GAME.lock().player_position.x;
            let player_y = GAME.lock().player_position.y;

            GAME.lock().camera.x = player_x - ((screen_w / 4) / 2);
            GAME.lock().camera.y = player_y - ((screen_h / 4) / 2);
        }
        GAME.lock().screen_w = canvas.output_size().unwrap().0 as i32;
        GAME.lock().screen_h = canvas.output_size().unwrap().1 as i32;
    }
}
