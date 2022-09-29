use crate::entity::entity::SpawnedEntity;
use crate::entity::entitys::player::PlayerEntity;
use crate::game::gamestate::GAME;
use crate::tile::tile::WorldTile;
use crate::tile::tiles::grass::GrassTile;
use perlin2d::PerlinNoise2D;
use sdl2::gfx::framerate::FPSManager;
use sdl2::pixels::Color;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::WindowContext;

use crate::tile::tiles::water::WaterTile;
use crate::world::world::WorldTemplate;

use super::assetmanager::AssetManager;

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
            asset_manager: Default::default(),
            fullscreen: false,
        };
    }
}

impl Window {
    pub fn render(&mut self) {
        let sdl_context = sdl2::init().expect("Failed to init SDL2");
        let video_subsystem = sdl_context.video().expect("Failed to get video subsystem");

        let window = video_subsystem
            .window(&self.title, 800, 600)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())
            .expect("Failed to create window");

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

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context
            .event_pump()
            .expect("Failed to create event punp");

        let mut world = WorldTemplate::new();

        info!("Generating world");
        let perlin = PerlinNoise2D::new(6, 10.0, 0.5, 1.0, 2.0, (100.0, 100.0), 0.5, 1231313);

        for x in 0..1000 {
            for y in 0..1000 {
                let noise = perlin.get_noise(x as f64, y as f64);

                if noise < -6.0 {
                    world.insert_tile(WorldTile::from_tile(x * 32, y * 32, WaterTile {}));
                } else {
                    world.insert_tile(WorldTile::from_tile(x * 32, y * 32, GrassTile {}));
                }
            }
        }

        world.spawn_entity(SpawnedEntity::from_entity(PlayerEntity {}));
        info!("World generated");

        canvas.set_scale(4.0, 4.0).expect("Failed to set scale");

        let mut fps = FPSManager::new();
        fps.set_framerate(60).expect("Framerate set failed");

        loop {
            // cam.x = GAME.lock().player_x - (screen_x/2);
            // cam.y = GAME.lock().player_y - (screen_y/2) ;

            crate::gfx::events::key_events(&mut event_pump);

            let old = sdl_context.timer().unwrap().ticks();
            Self::ajust_screen(&canvas);

            canvas.clear();
            world.update_entitys(&mut event_pump);
            world.render(&mut canvas, self.asset_manager.as_ref().unwrap());

            canvas.present();

            fps.delay();
            let new = sdl_context.timer().unwrap().ticks();

            info!("FPS: {}", 1000 / (new - old));
            // info!("Deltatime: {}", (new - old) as f32 * 0.4);
        }
    }

    pub fn load_assets_on_render(&mut self, manager: AssetManager) {
        self.load_assets = true;
        self.asset_manager = Some(manager);
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

            GAME.lock().camera.x = player_x - ((screen_w/4)/2);
            GAME.lock().camera.y = player_y - ((screen_h/4)/2);;
        }
        GAME.lock().screen_w = canvas.output_size().unwrap().0 as i32;
        GAME.lock().screen_h = canvas.output_size().unwrap().1 as i32;
    }
}
