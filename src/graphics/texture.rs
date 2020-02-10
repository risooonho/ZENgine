extern crate image;

use crate::assets::image_loader::ImageAsset;
use crate::assets::image_loader;
use std::collections::HashMap;
use core::sync::atomic::{AtomicBool, Ordering};

/// Only one AssetManager can be alive at time
/// Set to false by default (not alive)
static IS_TEXTURE_MANAGER_ALIVE: AtomicBool = AtomicBool::new(false);

const LEVEL: i32 = 0;
const BORDER: i32 = 0;
/*
pub struct TextureManager {
    textures: HashMap<String, Texture>
}

impl TextureManager {
    pub fn init() -> TextureManager {
        let was_alive = IS_TEXTURE_MANAGER_ALIVE.swap(true, Ordering::Relaxed);

        if !was_alive {
            TextureManager {
                textures: HashMap::new()
            }
        } else {
            panic!("Cannot create two instance of TextureManager");
        }        
    }

    pub fn get(&self, image_path: &str) -> &Texture {
        match self.textures.get(image_path) {
            Some(texture) => {                
                return texture;
            },
            _ => {
                self.textures.insert(
                    String::from(image_path), 
                    Texture::new(image_path, image_loader::load(image_path))
                ); 

                return self.get(image_path);
            }
        };
    }

    pub fn release(&mut self, name: &str) {
        self.textures.remove(name);
    }
}*/

pub struct Texture {
    width: u32,
    height: u32,

    texture_id: u32
}

impl Texture {
    pub fn new(name: &str, image: ImageAsset) -> Texture {
        let mut t = Texture {
            width: 1,
            height: 1,

            texture_id: 0
        };

        t.width = image.width;
        t.height = image.height;

        unsafe {
            gl::GenTextures(1, &mut t.texture_id);
            gl::BindTexture(gl::TEXTURE_2D, t.texture_id);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                LEVEL,
                gl::RGBA as i32,
                t.width as i32,
                t.height as i32,
                BORDER,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const gl::types::GLvoid
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        t
    }

    pub fn activate_and_bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
        }
    }
}