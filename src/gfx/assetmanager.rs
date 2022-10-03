use hashbrown::HashMap;
use sdl2::image::LoadTexture;
use sdl2::render::Texture;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;

pub struct AssetManager {
    pub queue: Option<Vec<String>>,
    pub textures: HashMap<String, Texture>,
}

pub struct AssetManagerTemplate {}

impl AssetManagerTemplate {
    pub fn create() -> AssetManager {
        AssetManager {
            queue: Default::default(),
            textures: HashMap::new(),
        }
    }
}
impl AssetManager {
    pub fn new_queue(&mut self, assets: Vec<String>) {
        self.queue = Some(assets);
    }

    pub fn load_assets_from_queue(&mut self, texture_creator: &TextureCreator<WindowContext>) {
        info!("Loading assets from queue");

        let mut i = 0;
        for asset_path in self.queue.as_ref().unwrap().iter() {
            i += 1;
            info!(
                "Loading asset: {} [{}/{}]",
                asset_path,
                i,
                self.queue.as_ref().unwrap().len()
            );

            let texture = match texture_creator.load_texture(asset_path) {
                Ok(result) => result,
                Err(err) => {
                    error!("Texture loading error: {}", err);
                    std::process::exit(-1);
                }
            };

            self.textures.insert(asset_path.to_string(), texture);
            let texture = match texture_creator.load_texture(asset_path) {
                Ok(result) => result,
                Err(err) => {
                    error!("Texture loading error: {}", err);
                    std::process::exit(-1);
                }
            };

            self.textures.insert(asset_path.to_string(), texture);
        }

        info!(
            "Loaded all assets! [{}/{}]",
            i,
            self.queue.as_ref().unwrap().len()
        );
    }

    pub fn clear_queue(&mut self) {
        self.queue = Some(vec![]);
    }

    pub fn get_texture(&self, asset_path: String) -> &Texture {
        self.textures
            .get(&asset_path)
            .expect("Failed to find texture")
    }
}
